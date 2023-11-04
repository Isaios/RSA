use std::u128;
use num_bigint::{BigUint, ToBigUint, RandBigInt};
use num_traits::{One, Zero, pow};
use rand::Rng;

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

pub fn u128pow(b: u128, e: u128) -> u128 {
    let mut res = b;
    for _ in 1..e {
        res *= b;
    }
    res
}

pub fn rmt_big_uint(n: &BigUint, k: usize) -> bool {
    let mut rng = rand::thread_rng();

    let n_minus_one: BigUint = n - 1u8;
    let zero: BigUint = Zero::zero();
    let ref one: BigUint = One::one();
    let ref two: BigUint = 2.to_biguint().unwrap();

    let mut d: BigUint = n_minus_one.clone();
    let mut s: BigUint = Zero::zero();
    while &d & one == zero {
        d >>= 1;
        s += 1u8;
    }

    for _ in 0..k {
        let mut a: BigUint;
        loop {
            a = rng.gen_biguint_below(&n_minus_one);
            if a < n_minus_one && a > One::one() {
                break;
            }
        }
        let mut x = a.modpow(&d, n);
        let mut y = zero.clone();

        let mut i: BigUint = zero.clone();
        while &i < &s {

            y = x.modpow(&two, n);
            if y == One::one() && x != One::one() && x != n_minus_one{
                return false;
            }
            x = y.clone();
            i += 1u8;
        }
        if &y != one {
            return false;
        }
    }
    true
}
