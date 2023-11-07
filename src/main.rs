mod primagen;
mod rsa;
mod parse;

use std::{fs::File, io::{Write, Read}, os::unix::prelude::FileExt, vec};

use num_bigint::{BigUint, ToBigUint};
use num_traits::{ToPrimitive, FromBytes};

fn main(){
    // parse::parse();
    
/*     {
        let num: BigUint = 234152u32.into();

        let mut file = File::create("test").unwrap();
        file.write(&num.to_bytes_be()).unwrap();
    }
    {
        let file = File::open("test").unwrap();
        let mut buffer = [0u8; 1];
        for byte in file.bytes() {
            println!("{:?}", byte);
        }
        let file2 = File::open("test").unwrap();
        file2.read_at(&mut buffer, 0).unwrap();
        let num: BigUint = BigUint::from_bytes_be(&buffer);
        
        println!("{:?}", num);
    } */

    {
        let mut file = File::create("test").unwrap();
        let buf: [u8; 13] = [101, 
                            110,
                            2, 0, 0, 0, 
                            1, 0, 0, 0, 
                            1, 1,
                            1];
        file.write(&buf).unwrap();
    }
    {
        let sig: FileSignature = read_bufsig("test".into()).unwrap();
        read_buf("test".into()).unwrap();
    }

}

enum ReadBuf {
    Encrypt {
        e: BigUint,
        n: BigUint,
    },
    Decrypt {
        n: BigUint,
        d: BigUint,
    },
    Bin,
}

enum FileSignature {
    Encrypt,
    Decrypt,
    Bin,
}

// file format: 
// first two bytes evaluate to the ascii values either representing e and n or n and d, indicating
// what file type is beeing read. After that the lengths in bytes of the two values follows, so
// that they can be read in properly
//
//   0   1   2   3   4   5   6   7   8   9
// |___|___|___|___|___|___|___|___|___|___|___->
//  id1 id2 <-----len1----> <----len2-----> bytes
fn read_bufsig(path: std::path::PathBuf) -> Result<FileSignature, Box<dyn std::error::Error>>{
    let ref mut file = File::open(path)?;
    let mut ids: [u8; 2] = Default::default();
    file.read_exact(&mut ids)?;

    if ids[0] == 101 && ids[1] == 110 {
        return Ok(FileSignature::Encrypt)
    } else if ids[0] == 110 && ids[1] == 100 {
        return Ok(FileSignature::Decrypt)
    } else {
        return Ok(FileSignature::Bin)
    }
}

fn read_buf(path: std::path::PathBuf) -> Result<ReadBuf, Box<dyn std::error::Error>> {
    let sig: FileSignature = read_bufsig(path.clone()).unwrap();
    let ref mut file = File::open(path)?;
    // read in 8 bytes to fill two u32 values containing the sizes
    let mut size_buf1: [u8; 4]= Default::default();
    let mut size_buf2: [u8; 4]= Default::default();
    match sig {
        FileSignature::Encrypt => {
            // read in both sizes
            file.read_exact_at(&mut size_buf1, 2)?;
            file.read_exact_at(&mut size_buf2, 6)?;

            let e_size: u32 = u32::from_le_bytes(size_buf1);
            let mut e_buf: Vec<u8> = vec![0u8; e_size.try_into()?];
            file.read_exact_at(&mut e_buf, 10)?;
            let e: BigUint = BigUint::from_le_bytes(&e_buf);
            println!("read e: {:?}", e);

            let n_size: u32 = u32::from_le_bytes(size_buf2);
            let mut n_buf: Vec<u8> = vec![0u8; n_size.try_into()?];
            file.read_exact_at(&mut n_buf, (9 + e_size).try_into()?)?;
            let n: BigUint = BigUint::from_bytes_le(&n_buf);
            println!("read n: {:?}", n)
        },
        FileSignature::Decrypt => {
            file.read_exact_at(&mut size_buf1, 2)?;
            file.read_exact_at(&mut size_buf2, 6)?;
        },
        FileSignature::Bin => {

        }
    }

    Ok(ReadBuf::Bin)
}
