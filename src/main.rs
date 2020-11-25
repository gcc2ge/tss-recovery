mod polynomial;

use curv::{
    cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS,
    elliptic::curves::traits::ECScalar, BigInt, FE,
};
use polynomial::lagrange_interpolation as lag;

fn main() {
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
