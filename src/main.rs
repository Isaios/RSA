use num_bigint::{BigInt, BigUint, ToBigUint};
use rsa::generate_keys;

mod primagen;
mod rsa;

fn main() {
    let (e, n, d) = generate_keys();
    println!("e: {:?}\nn: {:?}\nd: {:?}", e, n, d);

    let z: BigUint = 369u16.into();
    let c = z.modpow(&e, &n);
    println!("c: {:?}", c);

    println!("z: {z:?}");
    println!("decryption: {:?}", c.modpow(&d.to_biguint().unwrap(), &n));
}
