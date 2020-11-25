use curv::{
    cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS,
    elliptic::curves::traits::ECScalar, BigInt, FE,
};
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
                // points:(scale,index)
                if !xi.eq(x) {
                    // 使用x.0与index
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
pub fn sample_polynomial(t: &[usize], skip: usize) -> Vec<FE> {

    let mut coefficients = t
        .iter()
        .map(|i| {
            if i != &skip {
                ECScalar::new_random()
            } else {
                ECScalar::zero()
            }
        })
        .collect::<Vec<FE>>();

    // 生成多项式
    coefficients
}


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
    let g3 = reconstruct_at_index(&vec![0, 1], &vec![g1.clone(), g2.clone()], 3); // g(3)
    println!("g3 {:?}", g3.clone());
    // test
    // let g1_test = lag::reconstruct_at_index(&[1, 2], &[g2.clone(), g3.clone()], 1);
    // println!("g1_test {:?}", g1_test.clone());

    // // h(x)
    let h1: FE = ECScalar::zero(); // h(1)=0
    let h2: FE = ECScalar::new_random(); // h(2)
    let h3 = reconstruct_at_index(&vec![0, 1], &vec![h1.clone(), h2.clone()], 3); // h(3)
    println!("{:?}", h3);

    // // f'(x)=f(x)+g(x)+h(x)
    let f2 = secret_shares[1]
        .add(&h2.get_element())
        .add(&g2.get_element());
    let f3 = secret_shares[2]
        .add(&h3.get_element())
        .add(&g3.get_element());

    // // recovery
    let r = reconstruct_at_index(&vec![1, 2], &vec![f2, f3], 1);
    println!("recovery {:?}", r);
    assert_eq!(r, secret_shares[0].clone());
}

#[test]
fn test_sample_polynomial(){
    let secret: FE = ECScalar::new_random();

    let (vss_scheme, secret_shares) = VerifiableSS::share(1, 3, &secret);
    println!("{:?}",secret_shares[0]);

    let g=sample_polynomial(&[0,1],0);
    println!("{:?}",g);

    let g3=reconstruct_at_index(&[0,1], &g, 3);
    println!("{:?}",g3);

    let h=sample_polynomial(&[0,1],0);
    println!("{:?}",h);

    let h3=reconstruct_at_index(&[0,1], &h, 3);
    println!("{:?}",h3);

 // // f'(x)=f(x)+g(x)+h(x)
    let f2 = secret_shares[1]
        .add(&h[1].get_element())
        .add(&g[1].get_element());
    let f3 = secret_shares[2]
        .add(&h3.get_element())
        .add(&g3.get_element());

    // // recovery
    let r = reconstruct_at_index(&vec![1, 2], &vec![f2, f3], 1);
    println!("recovery {:?}", r);
    assert_eq!(r, secret_shares[0].clone());
}