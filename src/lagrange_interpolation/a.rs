use curv::{
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
    let lag_coef = (0..vec_len)
        .filter(|x| ((x + 1) != index))
        .map(|i| {
            let xi = &points[i];
            let yi = &values[i];
            let num: FE = ECScalar::from(&BigInt::one());
            let denum: FE = ECScalar::from(&BigInt::one());
            let num = points.iter().zip(0..vec_len).fold(num, |acc, x| {
                if i != x.1 && (x.1 + 1) != index {
                    let xj_sub_xi = x.0.sub(&index_scale.get_element());
                    acc * xj_sub_xi
                } else {
                    acc
                }
            });
            let denum = points.iter().zip(0..vec_len).fold(denum, |acc, x| {
                if i != x.1 && (x.1 + 1) != index {
                    let xj_sub_xi = x.0.sub(&xi.get_element());
                    acc * xj_sub_xi
                } else {
                    acc
                }
            });
            let denum = denum.invert();
            num * denum * yi
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