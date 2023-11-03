mod primagen;
use num_traits::pow;

fn main() {
   // primagen::tst();
    let n = 5;
    let k = 10;
  if primagen::rmt(n, k) {
      println!("{n} is prime, with the probality of error of 1 in {prob}", prob = pow(4,k.try_into().unwrap()))
  }
}
