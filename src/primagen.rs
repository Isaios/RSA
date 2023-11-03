use std::u128;
use std::time::Instant;

use num_bigint::{BigUint, ToBigUint, RandBigInt, BigInt, ToBigInt};
use num_traits::{One, Zero, pow};
use rand::Rng;
pub fn tst() {

    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let three: BigUint = 3.to_biguint().unwrap();
    let five: BigUint = 5.to_biguint().unwrap();
    let six: BigUint = 6.to_biguint().unwrap();

    let mut rng = rand::thread_rng();
    let size: u64 = 52;
    

    let start = Instant::now();

    let mut rnd = rng.gen_biguint(size);
    while !prime1(rnd.clone(), zero.clone(), one.clone(), two.clone(), three.clone(), five.clone(), six.clone()) {
        rnd = rng.gen_biguint(size);
    }
    println!("prime: {:?}", rnd);

    let duration = start.elapsed();


    println!("prime detection of generated numbers: {:?}", duration);
}

pub fn rmt(n: u128, k: u128) -> bool {
    let mut rng = rand::thread_rng();

    let mut d = n - 1;
    let mut s: u32 = 0;
    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }
    
    for _ in 0..k {
        let a: u128 = rng.gen_range(1..n-1);
        let mut x = u128pow(a, d) % n;
        let mut y = 0;
        for _ in 0..s {
            y = (x*x) % n;
            if y == 1 && x != 1 && x != n-1 {
                return false;
            }
            x = y;
        }
        if y != 1 {
            return false;
        }
    }

    true
}
fn u128pow(b: u128, e: u128) -> u128 {
    let mut res = b;
    for _ in 1..e {
        res *= b;
    }
    res
}

pub fn decompose(n: &BigUint) -> (BigUint, BigUint) {
    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let ref two = 2.to_biguint().unwrap();
    let mut d: BigUint = (n - 1u8).clone();
    let mut s: BigUint = Zero::zero();
    
    while &d % two == zero {
        d /= two;
        s += 1u8;
    }

    (d, s)
}

// n a d s
pub fn rmt_big_uint(n: &BigUint, a: &BigUint, d: &BigUint, s: &BigUint) -> bool {
    let n_minus_one: BigUint = n - 1u8;
    let mut x = a.modpow(d, n);
    let mut y = Zero::zero();
    let mut i: BigUint = One::one();
    let ref two: BigUint = 2.to_biguint().unwrap();
    
    while &i < s {
        y = x.modpow(two, n);

        if y == One::one() && x != One::one() && x != n_minus_one {
            return false;
        }

        x = y.clone();
        i += 1u8;
    }
    if y != One::one() {
        return false;
    }
    true
}

fn is_prime(n: u128) -> bool {
    //if n == 2 || n == 3 {
        //return true; optimisation?
    //}

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: u128 = 5;

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        
        i += 6;
    }

    true
}

pub fn prime(n: BigUint) -> bool {
    if n == 2.to_biguint().unwrap() || n == 3.to_biguint().unwrap() {
        return true;
    }

    if n <= 1.to_biguint().unwrap() || &n % &2.to_biguint().unwrap() == Zero::zero() || &n % &3.to_biguint().unwrap() == Zero::zero() {
        return false;
    }

    let mut i: BigUint = 5.to_biguint().unwrap();

    while &i * &i <= n {
        if &n % &i == Zero::zero() || &n % (&i + 2.to_biguint().unwrap()) == Zero::zero() {
            return false;
        }

        i += 6.to_biguint().unwrap();
    }

    return true;
}

fn prime1(n: BigUint, zero: BigUint, one: BigUint, two: BigUint, three: BigUint, five: BigUint, six: BigUint) -> bool {
    if n == two || n == three {
        return true;
    }

    if n <= one || &n % &two == zero || &n % &three == zero {
        return false;
    }

    let mut i: BigUint = five;

    while &i * &i <= n {
        if &n % &i == zero || &n % (&i + &two) == zero {
            return false;
        }

        i += &six;
    }

    return true;
}

