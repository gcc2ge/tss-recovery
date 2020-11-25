mod polynomial;

use curv::{
    cryptographic_primitives::secret_sharing::feldman_vss::VerifiableSS,
    elliptic::curves::traits::ECScalar, BigInt, FE,
};
use polynomial::lagrange_interpolation as lag;

fn main() {
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
    println!("g1 {:?}",g1.clone());
    let g2: FE = ECScalar::new_random(); // g(2)
    println!("g2 {:?}",g2.clone());
    let g3 = lag::reconstruct_at_index(&vec![0, 1], &vec![g1.clone(), g2.clone()], 3); // g(3)
    println!("g3 {:?}", g3.clone());
    // test
    // let g1_test = lag::reconstruct_at_index(&[1, 2], &[g2.clone(), g3.clone()], 1);
    // println!("g1_test {:?}", g1_test.clone());

    // // h(x)
    let h1: FE = ECScalar::zero(); // h(1)=0
    let h2: FE = ECScalar::new_random(); // h(2)
    let h3=lag::reconstruct_at_index(&vec![0,1], &vec![h1.clone(),h2.clone()], 3); // h(3)
    println!("{:?}",h3);

    // // f'(x)=f(x)+g(x)+h(x)
    let f2=secret_shares[1].add(&h2.get_element()).add(&g2.get_element());
    let f3=secret_shares[2].add(&h3.get_element()).add(&g3.get_element());

    // // recovery
    let r=lag::reconstruct_at_index(&vec![1,2], &vec![f2,f3], 1);
    println!("recovery {:?}",r);

}
