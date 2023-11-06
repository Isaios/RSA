//TODO:
//CLAP:
//  rsa_encrypt
//  rsa_decrypt
//File access

use num_bigint::BigUint;
use rsa::*;
use clap::{arg, Command};

mod primagen;
mod rsa;
use crate::primagen::*;
fn main() {
    let matches = Command::new("jmrsa")
        .version("1.0")
        .about("RSA toolbox")
        .subcommand(
            Command::new("encrypt")
            .arg(arg!(
                    -n --new ... "create new keys and use them further"
                    ))
            .arg(arg!( -t --text <VALUE>).required(true))
            )
        .subcommand(
            Command::new("decrypt")

            )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("encrypt") {
        let z = matches.get_one::<String>("text").parse::<BigUint>().unwrap();
;
        //let z = .parse::<BigUint>().unwrap();
        println!("z");
        if matches.get_flag("n") {

        }
    }
    if let Some(matches) = matches.subcommand_matches("decrypt") {
    }





    //let (e, n, d) = create_keys(generate_primes(1024, 500));
    //println!("e: {:?}\nn: {:?}\nd: {:?}", e, n, d);

    //let z: BigUint = 368u16.into();
    //let c = rsa_encrypt(&z, &e, &n);
    //println!("c: {:?}", c);

    //println!("z: {z:?}");
    //println!("decryption: {:?}", rsa_decrypt(&c, &d, &n)); 
}
