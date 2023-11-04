//!!NEEDS BIGINTS TO WORK WELL!!
use crate::primagen::*;
use modpow::modpow;

// p & q are prime integers 
// e is coprime with phi(n) and can also be prime, since every prime is coprime with any number.(generating coprimes specifically can be faster, but is a hustle)
// returns public key n, input e is the other  public key, d is the private key 
pub fn rsa_make_keys(p:i128, q:i128, e:i128) -> (i128, i128){ 
    let d = (1 - (p - 1) * (q - 1)) / e; // maybe needs float?
    let n = p*q;
    (d, n)
}

// z = single i128 to encrypt
pub fn rsa_encrypt(e:i128, n: i128, z: i128) -> i128 {
return i128pow(z, e) % n;

}
//c = single i128 to decrypt
pub fn rsa_decrypt(c:i128, d: i128, n: i128) -> i128 {
return i128pow(c, d) % n;
}
