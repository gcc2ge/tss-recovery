mod polynomial;

use curv::{
    cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS,
    elliptic::curves::traits::ECScalar, BigInt, FE,
};
use polynomial::lagrange_interpolation as lag;

fn main() {
    // let a=lag::recover_lost_share(3, 5,  1);
    // println!("{:?}",a);

    let secret: FE = ECScalar::new_random();

    let (vss_scheme, secret_shares) = VerifiableSS::share(2, 5, &secret);
    println!("{:?}", secret_shares[0]);

    let g = lag::recover_lost_share(3, 5,  0);
    println!("g {:?}", g);

    let h = lag::recover_lost_share(3, 5,  0);
    println!("h {:?}", h);

    let v = lag::recover_lost_share(3, 5,  0);
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
        .add(&h[3].1.get_element())
        .add(&g[3].1.get_element())
        .add(&v[3].1.get_element());

    // // recovery
    let r = lag::reconstruct_at_index(&vec![1, 2, 3], &vec![f2, f3, f4], 1);
    println!("recovery {:?}", r);
    assert_eq!(r, secret_shares[0].clone());
}
