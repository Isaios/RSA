use std::num;

use num_bigint::{ToBigUint, BigUint};
use primagen::{decompose, rmt};
use rand::Rng;

use crate::primagen::rmt_big_uint;

mod primagen;

fn main() {
    let num: u128 = 49;
    if rmt(num, 5) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}
