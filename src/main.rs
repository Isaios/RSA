use num_bigint::{ BigUint, RandBigInt};
use primagen::rmt_bu2;

mod primagen;
mod rsa;
fn main() {
    let size: u64 = 512;

    let mut n: BigUint = rand::thread_rng().gen_biguint(size);
    let mut rng = rand::thread_rng();
    {
        let start = std::time::Instant::now();
        
        while !rmt_bu2(&n, 3) {
            n = rng.gen_biguint(size);
        }

        let d = start.elapsed();
        println!("rmt prime: {} in {:?}", n, d);
    }
}
