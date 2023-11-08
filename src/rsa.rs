use crate::primagen::rmt;
use num_bigint::{BigInt, ToBigUint, BigUint};
use num_traits::{One, Zero};

/// Encrypts the value z using the rsa cryptosystem. Returns the encrypted value
///
/// # Arguments
///
/// * `z` - value to encrypt
/// * `e` - encryption exponent
/// * `n` - encryption modulus
pub fn rsa_encrypt(z: &BigUint , e: &BigUint, n: &BigUint) -> BigUint {
    z.modpow(e, n)
}

/// Decrypts the value c using the rsa cryptosystem. Returns the decrypted value
///
/// # Arguments
///
/// * `c` - value to decrypt
/// * `d` - decryption exponent
/// * `n` - decryption modulus
pub fn rsa_decrypt(c: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    c.modpow(d, n)
}


/// Generates rsa keys from the given tuple of primes 
/// 
/// # Arguments
/// 
/// * `primes_tupel` - tupel holding the two primes p and q needed for key creation
///
/// # Examples
/// ```
/// // create the key components from two newly generated primes
/// let (e, n, d) = create_keys(generate_primes(1024, 500));
/// ```
pub fn create_keys(primes_tupel: (BigUint, BigUint)) -> (BigUint, BigUint, BigUint){
    let (p, q) = primes_tupel;
    let one: BigUint = One::one();
    let n = &p * &q;
    let phi_n = (p-&one) * (q-&one);
    let mut d: BigInt = Zero::zero();
    let mut k: BigInt = Default::default();
    let mut e = 65537.to_biguint().unwrap(); 
    while d <= Zero::zero() {
        
        if gcd_extended_iterative(e.clone().into(), phi_n.clone().into(), &mut d, &mut k) == 1u8.into() && &d >= &1u8.into() {break;}
        e += &one; 
        while !rmt(&e, 500) {
           e += &one; 
        } 
    }
    (e, n, d.to_biguint().unwrap())
}

/// Performs the extended euclidean algorithm to determine the gcd as well as the coefficients.
/// Returns the gcd of both numbers
/// 
/// # Arguments
///
/// * `a` - first number
/// * `b` - second number
/// * `d` - reference to store the d value
/// * `k` - reference to store the k value
///
/// # Examples
/// ```
/// let mut a = 23.to_bigint().unwrap();
/// let mut b = 120.to_bigint().unwrap();
///
/// let mut d = Default::default;
/// let mut k = Default::default;
///
/// assert_eq![gcd_extended_iterative(a, b, &mut d, &mut k), 1];
/// assert_eq![d, 47.into()];
/// assert_eq![k, -9.into()];
/// ```
fn gcd_extended_iterative(mut a: BigInt, mut b: BigInt, d: &mut BigInt, k: &mut BigInt) -> BigInt {
    *d = 1u8.into();
    *k = 0u8.into();
    let mut u: BigInt = 0u8.into();
    let mut v: BigInt = 1u8.into();

    while b != Zero::zero() {
        let q = &a / &b;
        let b1 = b.clone();
        b = &a - &q * &b;
        a = b1;
        let u1 = u.clone();
        u = &*d - &q * u;
        *d = u1;
        let v1 = v.clone();
        v = &*k - &q * v;
        *k = v1;
    }
    a
}
