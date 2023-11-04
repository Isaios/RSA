mod primagen;
mod rsa;

fn main() {

    let primes = primagen::generate(1000, 500, 3);
    for prime in primes {
        println!("{:?}", prime);
    }

}
