use std::u128;
use std::time::Instant;

use num_bigint::{BigUint, ToBigUint, RandBigInt};
use num_traits::{One, Zero};

pub fn tst() {
/*     let num: u128 = 100000;

    let start = Instant::now();
    for n in 1..num {
        is_prime(n);
    }
    let duration = start.elapsed();
    println!("standart prime detection{:?}", duration); */

    /* let start = Instant::now();
    for n in 1..num {
        prime(n.to_biguint().unwrap());
    }
    let duration = start.elapsed();
    println!("\n{:?}", duration); */

    let zero: BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two: BigUint = 2.to_biguint().unwrap();
    let three: BigUint = 3.to_biguint().unwrap();
    let five: BigUint = 5.to_biguint().unwrap();
    let six: BigUint = 6.to_biguint().unwrap();

    /* let start = Instant::now();
    for n in 1..num {
        prime1(n.to_biguint().unwrap(), zero.clone(), one.clone(), two.clone(), three.clone(), five.clone(), six.clone());
    }
    let duration = start.elapsed();
    println!("optimized prime BigUint detection: {:?}", duration); */

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

fn is_prime(n: u128) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
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

fn prime(n: BigUint) -> bool {
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