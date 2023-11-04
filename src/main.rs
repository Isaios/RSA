use std::num;

use num_bigint::{ToBigUint, BigUint};
use primagen::{decompose, rmt};
use rand::Rng;
use rsa::*;
use crate::primagen::rmt_big_uint;

mod primagen;
mod rsa;
fn main() {
    let p:i128 = 3;
    let q:i128 = 7;
    let e:i128 = 5;
    let (d, n) = rsa_make_keys(p,q,e);
    let crypt = rsa_encrypt(e, n, 220705);
    println!("crypt of 220705: {}",crypt);
    println!("decrypt of {}: {}",crypt, rsa_decrypt(crypt, d, n));
    




    let num: u128 = 49;
    if rmt(num, 5) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}
