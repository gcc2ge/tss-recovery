mod polynomial;

use curv::{
    cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS,
    elliptic::curves::traits::ECScalar, BigInt, FE,
};
use polynomial::lagrange_interpolation as lag;

fn main() {
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
    let r = lag::reconstruct_at_index(&vec![1, 2,3], &vec![f2, f3,f4], 1);
    println!("recovery {:?}", r);
    assert_eq!(r, secret_shares[0].clone());
}
