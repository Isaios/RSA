use num_bigint::{BigInt, BigUint, ToBigUint};
use rsa::generate_keys;

mod primagen;
mod rsa;

fn main() {
    let (e, n, d) = generate_keys();
    println!("e: {:?}, n: {:?}, d: {:?}", e, n, d);

    let z: BigUint = 369u16.into();
    let c = z.modpow(&e, &n);
    println!("c: {:?}", c);

    println!("dectypted: {:?}", c.modpow(&d.to_biguint().unwrap(), &n));
}
