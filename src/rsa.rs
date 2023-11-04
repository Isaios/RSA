//!!NEEDS BIGINTS TO WORK WELL!!
use crate::primagen::*;

// p & q are prime integers 
// e is coprime with phi(n) and can also be prime, since every prime is coprime with any number.(generating coprimes specifically can be faster, but is a hustle)
// returns public key n, input e is the other  public key, d is the private key 
pub fn rsa_make_keys(p:u128, q:u128, e:u128) -> (u128, u128){ 
    let d = (1 - (p - 1) * (q - 1)) / e; // maybe needs float?
    let n = p*q;
    (d, n)
}

// z = single u128 to encrypt
pub fn rsa_encrypt(e:u128, n: u128, z: u128) -> u128 {
    return u128pow(z, e) % n;

}
//c = single u128 to decrypt
pub fn rsa_decrypt(c:u128, d: u128, n: u128) -> u128 {
    return u128pow(c, d) % n;
}
