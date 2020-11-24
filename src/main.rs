use curv::{
    cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS,
    elliptic::curves::traits::ECScalar, BigInt, FE,
};
use lagrange_interpolation::a;

mod lagrange_interpolation;

fn main() {
    let secret: FE = ECScalar::new_random();

    let (vss_scheme, secret_shares) = VerifiableSS::share(1, 3, &secret);

    let mut shares_vec = Vec::new();
    shares_vec.push(secret_shares[0].clone());
    shares_vec.push(secret_shares[1].clone());
    shares_vec.push(secret_shares[2].clone());

    println!("secret {:?}",secret);
    println!("s1 {:?}",secret_shares[0].clone());
    println!("s2 {:?}",secret_shares[1].clone());
    println!("s3 {:?}",secret_shares[2].clone());

    let secret_reconstructed =a::reconstruct_at_index(&vec![0, 1,2], &shares_vec,3);
    println!("recover {:?}",secret_reconstructed);

}
