use num_bigint::{BigInt, BigUint, ToBigUint};
use rsa::*;

mod primagen;
mod rsa;

fn main() {
    let (e, n, d) = generate_keys();
    println!("e: {:?}\nn: {:?}\nd: {:?}", e, n, d);

    let z: BigUint = 368u16.into();
    let c = rsa_encrypt(&z, &e, &n);
    println!("c: {:?}", c);

    println!("z: {z:?}");
    println!("decryption: {:?}", rsa_decrypt(&c, &d, &n)); 
}
