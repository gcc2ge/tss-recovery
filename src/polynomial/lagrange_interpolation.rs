use curv::{elliptic::curves::traits::ECScalar, BigInt, FE};
//modify begin
pub fn reconstruct_at_index(indices: &[usize], shares: &[FE], index: usize) -> FE {
    assert_eq!(shares.len(), indices.len());
    // add one to indices to get points
    let points = indices
        .iter()
        .map(|i| {
            let index_bn = BigInt::from(*i as u32 + 1 as u32);
            ECScalar::from(&index_bn)
        })
        .collect::<Vec<FE>>();
    lagrange_interpolation_at_index(&points, &shares, index)
}

pub fn lagrange_interpolation_at_index(points: &[FE], values: &[FE], index: usize) -> FE {
    let vec_len = values.len();

    assert_eq!(points.len(), vec_len);
    let index_scale: FE;
    if index == 0 {
        index_scale = ECScalar::zero();
    } else {
        let index_bn = BigInt::from(index as u32);
        index_scale = ECScalar::from(&index_bn);
    }
    let zero = ECScalar::zero();

    let lag_coef = (0..vec_len)
        .map(|i| {
            let xi = &points[i];
            let yi = &values[i];
            let num: FE = ECScalar::from(&BigInt::one());
            let denum: FE = ECScalar::from(&BigInt::one());
            let num = points.iter().fold(num, |acc, x| {
                if !xi.eq(x) {
                    let xj_sub_xi = x.sub(&index_scale.get_element());
                    acc * xj_sub_xi
                } else {
                    acc
                }
            });
            let denum = points.iter().fold(denum, |acc, x| {
                if !xi.eq(x) {
                    let xj_sub_xi = x.sub(&xi.get_element());
                    acc * xj_sub_xi
                } else {
                    acc
                }
            });
            let denum = denum.invert();
            if yi.eq(&zero) {
                zero
            } else {
                num * denum * yi
            }
        })
        .collect::<Vec<FE>>();
    let mut lag_coef_iter = lag_coef.iter();
    let head = lag_coef_iter.next().unwrap();
    let tail = lag_coef_iter;
    tail.fold(head.clone(), |acc, x| acc.add(&x.get_element()))
}

//modify end

// Bob 生成g(x) ,g(1)=0, f'(x)=f(x)+g(x)+h(x)
// g(x): g(1)=0 g(2)=r,g(3)=r
// Caler 生成h(x) ,h(1)=0, f'(x)=f(x)+g(x)+h(x)
// h(x): h(1)=0 h(2)=r,h(3)=r

// 生成的值给 Alice f'(2),f'(3),alice 通过reconstruct_at_index生成 f'(1)，恢复secret

// sample 生成多项式 g(x)
// 1 2 3
// 1 => 0
// 2 => r
// 3 => r
// t [1,2] ,skip 1
pub fn sample_polynomial(t: usize, lost: usize) -> Vec<(usize, FE)> {
    let n: usize;
    let insert: bool;
    if t <= lost {
        n = t - 1;
        insert = true;
    } else {
        n = t;
        insert = false;
    }

    let mut coefficients = (0..n)
        .map(|x| {
            if !insert && x == lost {
                (x, ECScalar::zero())
            } else {
                (x, ECScalar::new_random())
            }
        })
        .collect::<Vec<(usize, FE)>>();

    if insert {
        coefficients.push((lost, ECScalar::zero()));
    }

    // 生成多项式
    coefficients
}

fn find_vec(g: &Vec<(usize, FE)>, i: usize) -> Option<&(usize, FE)> {
    let mut iter = g.iter();
    iter.find(|x| x.0 == i)
}

pub fn recover_lost_share(t: usize, n: usize, lost: usize) -> Vec<(usize, FE)> {
    let mut g = sample_polynomial(t, lost); 
    let mut t = (0..n)
        .map(|x| match find_vec(&g, x) {
            None => {
                let points = g.iter().map(|x| x.0).collect::<Vec<usize>>();
                let shares = g.iter().map(|x| x.1).collect::<Vec<FE>>();

                (x, reconstruct_at_index(&points, &shares, x + 1))
            }
            Some(&(a, b)) => (a, b),
        })
        .collect::<Vec<(usize, FE)>>();

    t
}

#[cfg(test)]
mod tests {
    use crate::polynomial::lagrange_interpolation as lag;
    use curv::{
        cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS,
        elliptic::curves::traits::ECScalar, BigInt, FE,
    };

    #[test]
    fn test_recover() {
        let secret: FE = ECScalar::new_random();

        let (vss_scheme, secret_shares) = VerifiableSS::share(1, 3, &secret);

        let mut shares_vec = Vec::new();
        shares_vec.push(secret_shares[0].clone());
        shares_vec.push(secret_shares[1].clone());
        shares_vec.push(secret_shares[2].clone());

        println!("secret {:?}", secret);
        println!("s1 {:?}", secret_shares[0].clone());
        println!("s2 {:?}", secret_shares[1].clone());
        println!("s3 {:?}", secret_shares[2].clone());

        // let secret_reconstructed = lag::reconstruct_at_index(&vec![0, 1, 2], &shares_vec, 0);
        // println!("recover {:?}", secret_reconstructed);

        // 生成 g(x) h(x)

        // g(x)
        let g1: FE = ECScalar::zero(); //g(1)=0
        println!("g1 {:?}", g1.clone());
        let g2: FE = ECScalar::new_random(); // g(2)
        println!("g2 {:?}", g2.clone());
        let g3 = lag::reconstruct_at_index(&vec![0, 1], &vec![g1.clone(), g2.clone()], 3); // g(3)
        println!("g3 {:?}", g3.clone());
        // test
        // let g1_test = lag::reconstruct_at_index(&[1, 2], &[g2.clone(), g3.clone()], 1);
        // println!("g1_test {:?}", g1_test.clone());

        // // h(x)
        let h1: FE = ECScalar::zero(); // h(1)=0
        let h2: FE = ECScalar::new_random(); // h(2)
        let h3 = lag::reconstruct_at_index(&vec![0, 1], &vec![h1.clone(), h2.clone()], 3); // h(3)
        println!("{:?}", h3);

        // // f'(x)=f(x)+g(x)+h(x)
        let f2 = secret_shares[1]
            .add(&h2.get_element())
            .add(&g2.get_element());
        let f3 = secret_shares[2]
            .add(&h3.get_element())
            .add(&g3.get_element());

        // // recovery
        let r = lag::reconstruct_at_index(&vec![1, 2], &vec![f2, f3], 1);
        println!("recovery {:?}", r);
        assert_eq!(r, secret_shares[0].clone());
    }

    #[test]
    fn test_sample_polynomial_2_3() {
        let secret: FE = ECScalar::new_random();

        let (vss_scheme, secret_shares) = VerifiableSS::share(1, 3, &secret);
        println!("{:?}", secret_shares[0]);

        let g = lag::sample_polynomial(2, 0);
        println!("g {:?}", g);

        let g3 = lag::reconstruct_at_index(&[g[0].0, g[1].0], &[g[0].1, g[1].1], 3);
        println!("g3 {:?}", g3);

        let h = lag::sample_polynomial(2, 0);
        println!("h {:?}", h);

        let h3 = lag::reconstruct_at_index(&[h[0].0, h[1].0], &[h[0].1, h[1].1], 3);
        println!("h3 {:?}", h3);

        // // f'(x)=f(x)+g(x)+h(x)
        let f2 = secret_shares[1]
            .add(&h[1].1.get_element())
            .add(&g[1].1.get_element());
        let f3 = secret_shares[2]
            .add(&h3.get_element())
            .add(&g3.get_element());

        // // recovery
        let r = lag::reconstruct_at_index(&vec![1, 2], &vec![f2, f3], 1);
        println!("recovery {:?}", r);
        assert_eq!(r, secret_shares[0].clone());
    }

    #[test]
    fn test_sample_polynomial_3_5() {
        let secret: FE = ECScalar::new_random();

        let (vss_scheme, secret_shares) = VerifiableSS::share(2, 5, &secret);
        println!("{:?}", secret_shares[0]);

        let g = lag::sample_polynomial(3, 0);
        println!("g {:?}", g);

        let g4 = lag::reconstruct_at_index(&[g[0].0, g[1].0, g[2].0], &[g[0].1, g[1].1, g[2].1], 4);
        println!("g4 {:?}", g4);

        let h = lag::sample_polynomial(3, 0);
        println!("h {:?}", h);

        let h4 = lag::reconstruct_at_index(&[h[0].0, h[1].0, h[2].0], &[h[0].1, h[1].1, h[2].1], 4);
        println!("h4 {:?}", h4);

        let v = lag::sample_polynomial(3, 0);
        println!("v {:?}", v);

        let v4 = lag::reconstruct_at_index(&[v[0].0, v[1].0, v[2].0], &[v[0].1, v[1].1, v[2].1], 4);
        println!("v4 {:?}", v4);

        // // f'(x)=f(x)+g(x)+h(x)
        let f2 = secret_shares[1]
            .add(&h[1].1.get_element())
            .add(&g[1].1.get_element())
            .add(&v[1].1.get_element());
        let f3 = secret_shares[2]
            .add(&h[2].1.get_element())
            .add(&g[2].1.get_element())
            .add(&v[2].1.get_element());
        let f4 = secret_shares[3]
            .add(&h4.get_element())
            .add(&g4.get_element())
            .add(&v4.get_element());

        // // recovery
        let r = lag::reconstruct_at_index(&vec![1, 2, 3], &vec![f2, f3, f4], 1);
        println!("recovery {:?}", r);
        assert_eq!(r, secret_shares[0].clone());
    }

    #[test]
    fn test_recover_lost_share_3_5() {
        let secret: FE = ECScalar::new_random();

        let (vss_scheme, secret_shares) = VerifiableSS::share(2, 5, &secret);
        println!("{:?}", secret_shares[0]);

        let g = lag::recover_lost_share(3, 5, 0);
        println!("g {:?}", g);

        let h = lag::recover_lost_share(3, 5, 0);
        println!("h {:?}", h);

        let v = lag::recover_lost_share(3, 5, 0);
        println!("v {:?}", v);

        // // f'(x)=f(x)+g(x)+h(x)
        let f2 = secret_shares[1]
            .add(&h[1].1.get_element())
            .add(&g[1].1.get_element())
            .add(&v[1].1.get_element());
        let f3 = secret_shares[2]
            .add(&h[2].1.get_element())
            .add(&g[2].1.get_element())
            .add(&v[2].1.get_element());
        let f4 = secret_shares[3]
            
            .add(&g[3].1.get_element())
            .add(&h[3].1.get_element())
            .add(&v[3].1.get_element());

        // // recovery
        let r = lag::reconstruct_at_index(&vec![1, 2, 3], &vec![f2, f3, f4], 1);
        println!("recovery {:?}", r);
        assert_eq!(r, secret_shares[0].clone());
    }
    

    #[test]
    fn test_recover_lost_share_2_3() {
        // let secret: FE = ECScalar::new_random();

        // let (vss_scheme, secret_shares) = VerifiableSS::share(1, 3, &secret);
        // println!("{:?}", secret_shares[0]);


        let s1_b=BigInt::from_str_radix("dbe746278c26d078e5e58f40e80f6d374223d4d183232a1b3d39d1f830e161b8",16).unwrap();
        let s1:FE=ECScalar::from(&s1_b);

        let s2_b=BigInt::from_str_radix("a2cbf0a406754a0969bd77f46b9f53290d8966f684aa0fb9ce629f148afa4455",16).unwrap();
        let s2:FE=ECScalar::from(&s2_b);

        let s3_b=BigInt::from_str_radix("69b09b2080c3c399ed9560a7ef2f391ad8eef91b8630f5585f8b6c30e51326f2",16).unwrap();
        let s3:FE=ECScalar::from(&s3_b);

        let g = lag::recover_lost_share(2, 3, 0);
        println!("g {:?}", g);

        let h = lag::recover_lost_share(2, 3, 0);
        println!("h {:?}", h);


        // // f'(x)=f(x)+g(x)+h(x)
        let f2 = s2
            .add(&h[1].1.get_element())
            .add(&g[1].1.get_element());

        let f3 = s3
            .add(&h[2].1.get_element())
            .add(&g[2].1.get_element());

    

        // // recovery
        let r = lag::reconstruct_at_index(&vec![1, 2], &vec![f2, f3], 1);
        println!("recovery {:?}", r);
        assert_eq!(r, s1.clone());
    }
}
