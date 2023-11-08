use bus;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_cpus;
use num_traits::{One, Zero};
use std::sync::mpsc;
use std::thread;

/// Implementation of the miller-rabin primality test, returns true if the number should be prime
///
/// # Arguments
/// * `n` - number to test
/// * `k` - number of random numbers n should be tested against
///
/// # Examples
/// ```
/// let num: BigUint = 65537.to_biguint().unwrap();
/// assert_eq![rmt(n: num, k: 500), true];
/// ```
pub fn rmt(n: &BigUint, k: usize) -> bool {
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
            if y == One::one() && x != One::one() && x != n_minus_one {
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

/// Function to generate prime numbers using the miller-rabin probalistic primality test
/// IMPORTANT: large size values needed for encryption will greatly increase time needed for 
/// computation; release mode, as well as a reasonably powerful cpu are neccessary
///
/// # Arguments
///
/// * `size` - usize indicating the bit size of the primes. Has to be at least 3
/// * `k` - usize indicating the number of checks performed by the miller-rabin test
///
/// # Examples
///
/// ```
/// // generating a tuple with 2 primes with the bit length of 1000 using 500 checks per prime
/// let primes: Vec<BigUint> = generate(1000, 500);
/// ```
pub fn generate_primes(size: usize, k: usize) -> (BigUint, BigUint) {
    // create a bus for handling the cancellation of all threads, a channel for transmition of the
    // primes to the main thread and a thread array holding all handles
    let mut bus: bus::Bus<bool> = bus::Bus::new(1);
    let (sender, receiver): (mpsc::Sender<BigUint>, mpsc::Receiver<BigUint>) = mpsc::channel();
    let mut threads = vec![];

    let mut primes: Vec<BigUint> = vec![];
    let mut prime_counter = 0;
    let count = 2;

    // create as many threads as there are cpu threads
    for _ in 0..num_cpus::get() {
        let mut bus_rx: bus::BusReader<bool> = bus.add_rx();
        let tx: mpsc::Sender<BigUint> = sender.clone();
        // spawn the thread and move variables into it
        let thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut n: BigUint;
            // while no signal was received via the bus, proceed to generate and test numbers
            while !match bus_rx.try_recv() {
                Ok(r) => r,
                _ => false,
            } {
                n = rng.gen_biguint(size.clone().try_into().unwrap());
                // is number is prime, send it over the std::sync::mpsc channel
                if rmt(&n, k) {
                    tx.send(n).unwrap();
                }
            }
        });
        // collect all of the threads in the vector to join them later
        threads.push(thread);
    }

    // try receiving the primes from the threads finishing generating
    loop {
        match receiver.try_recv() {
            Ok(prime) => {
                primes.push(prime);
                prime_counter += 1
            }
            _ => (),
        }
        // as soon as the desired amount of primes has been received broadcast the cancellation signal and quit the loop
        if prime_counter == count {
            bus.broadcast(true);
            break;
        }
    }

    // wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    (primes[0].clone(), primes[1].clone())
}

pub fn gen_primes(size: usize, k: usize, count: usize) {
    let mut bus: bus::Bus<bool> = bus::Bus::new(1);
    let (sender, receiver): (mpsc::Sender<BigUint>, mpsc::Receiver<BigUint>) = mpsc::channel();
    let mut threads = vec![];

    let mut prime_counter = 0;

    // create as many threads as there are cpu threads
    for _ in 0..num_cpus::get() {
        let mut bus_rx: bus::BusReader<bool> = bus.add_rx();
        let tx: mpsc::Sender<BigUint> = sender.clone();
        // spawn the thread and move variables into it
        let thread = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut n: BigUint;
            // while no signal was received via the bus, proceed to generate and test numbers
            while !match bus_rx.try_recv() {
                Ok(r) => r,
                _ => false,
            } {
                n = rng.gen_biguint(size.clone().try_into().unwrap());
                // is number is prime, send it over the std::sync::mpsc channel
                if rmt(&n, k) {
                    tx.send(n).unwrap();
                }
            }
        });
        // collect all of the threads in the vector to join them later
        threads.push(thread);
    }

    // try receiving the primes from the threads finishing generating
    loop {
        match receiver.try_recv() {
            Ok(prime) => {
                println!("{:?}", prime);
                prime_counter += 1
            }
            _ => (),
        }
        // as soon as the desired amount of primes has been received broadcast the cancellation signal and quit the loop
        if prime_counter == count {
            bus.broadcast(true);
            break;
        }
    }

    // wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }
}
