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
    println!("{:?}", secret_shares[1]);

    let g = lag::recover_lost_share(3, 5,  1);
    println!("g {:?}", g);

    let h = lag::recover_lost_share(3, 5,  1);
    println!("h {:?}", h);

    let v = lag::recover_lost_share(3, 5,  1);
    println!("v {:?}", v);

    // // f'(x)=f(x)+g(x)+h(x)
    let f1 = secret_shares[0]
        .add(&h[0].1.get_element())
        .add(&g[0].1.get_element())
        .add(&v[0].1.get_element());
    let f3 = secret_shares[2]
        .add(&h[2].1.get_element())
        .add(&g[2].1.get_element())
        .add(&v[2].1.get_element());
    let f4 = secret_shares[3]
        .add(&h[3].1.get_element())
        .add(&g[3].1.get_element())
        .add(&v[3].1.get_element());

    // // recovery
    let r = lag::reconstruct_at_index(&vec![0, 2, 3], &vec![f1, f3, f4], 2);
    println!("recovery {:?}", r);
    assert_eq!(r, secret_shares[1].clone());
}
