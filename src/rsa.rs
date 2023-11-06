//!!NEEDS BIGINTS TO WORK WELL!!

use crate::primagen::*;
use num_bigint::{BigInt, ToBigInt, ToBigUint, BigUint};
use num_traits::{One, Zero};

// c = single u128 to encrypt
pub fn rsa_encrypt(c: &BigUint , d: &BigUint, n: &BigUint) -> BigUint {
    c.modpow(&d.to_biguint().unwrap(), n)
}
//c = single u128 to decrypt
pub fn rsa_decrypt(c: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    c.modpow(d, n)
}

pub fn generate_keys() -> (BigUint, BigUint, BigUint) {
    let one: BigUint = One::one();

    let start = std::time::Instant::now();
    let primes = generate(1000, 500, 2);
    let p = primes[0].clone();
    let q = primes[1].clone();
    println!("generated primes in: {:?}", start.elapsed());

    
    let n = &p * &q;
    let phi_n = (p-&one) * (q-&one);
    let mut d: BigInt = Zero::zero();
    let mut k: BigInt = Default::default();
    let mut e = 65537.to_biguint().unwrap(); 
    while d <= Zero::zero() {
        
        if gcd_extended_iterative(e.clone().into(), phi_n.clone().into(), &mut d, &mut k) == 1u8.into() && &d >= &1u8.into() {break;}
        e += &one; 
        while !rmt_big_uint(&e, 500) {
           e += &one; 
        } 
    }
    (e, n, d.to_biguint().unwrap())

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
