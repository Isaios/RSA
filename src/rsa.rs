//!!NEEDS BIGINTS TO WORK WELL!!

use crate::primagen::*;
use num_bigint::{BigInt, ToBigInt, ToBigUint, BigUint};
use num_traits::*;

// p & q are prime integers 
// e is coprime with phi(n) and can also be prime, since every prime is coprime with any number.(generating coprimes specifically can be faster, but is a hustle)
// returns public key n, input e is the other  public key, d is the private key 
pub fn rsa_make_keys(p:u128, q:u128, e:u128) -> (u128, u128){ 
    let d = (1 - (p - 1) * (q - 1)) / e; // maybe needs float?
    let n = p*q;
    (d, n)
}

// z = single u128 to encrypt
pub fn rsa_encrypt(e:u128, n: u128, z: u128) -> u128 {
    return u128pow(z, e) % n;

}
//c = single u128 to decrypt
pub fn rsa_decrypt(c:u128, d: u128, n: u128) -> u128 {
    return u128pow(c, d) % n;
}

pub struct PublicPair {
    pub e: BigUint,
    pub n: BigUint,
}

pub struct KeyPair {
    pub public: PublicPair,
    pub private: BigUint,
}

pub fn generate_keys() -> (BigUint, BigUint, BigInt) {
    let one: BigUint = One::one();

    let start = std::time::Instant::now();
    let primes = generate(1000, 500, 2);
    let p = primes[0].clone();
    let q = primes[1].clone();
    println!("generated primes in: {:?}", start.elapsed());

    let e = 65537.to_biguint().unwrap();

    let n = &p * &q;
    let phi_n = (p-&one) * (q-&one);

    let mut d: BigInt = Default::default();
    let mut k: BigInt = Default::default();
    let gcd: BigInt = gcd_extended_iterative(e.clone().into(), phi_n.into(), &mut d, &mut k);

    (e, n, d)
}

pub fn gcd_extended_iterative(mut a: BigInt, mut b: BigInt, s: &mut BigInt, t: &mut BigInt) -> BigInt {
    *s = 1u8.into();
    *t = 0u8.into();
    let mut u: BigInt = 0u8.into();
    let mut v: BigInt = 1u8.into();

    while b != Zero::zero() {
        let q = &a / &b;
        let b1 = b.clone();
        b = &a - &q * &b;
        a = b1;
        let u1 = u.clone();
        u = &*s - &q * u;
        *s = u1;
        let v1 = v.clone();
        v = &*t - &q * v;
        *t = v1;
    }
    a
}
