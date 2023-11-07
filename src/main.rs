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
        let buf: [u8; 13] = [110, 
                            100,
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

enum ReadBuf {
    Encrypt {
        e: BigUint,
        n: BigUint,
    },
    Decrypt {
        n: BigUint,
        d: BigUint,
    },
    Bin {
        buf: BigUint,
    },
}

fn read_buf(path: std::path::PathBuf) -> Result<ReadBuf, Box<dyn std::error::Error>> {
    let sig: FileSignature = read_bufsig(path.clone()).unwrap();
    let ref mut file = File::open(path)?;
    match sig {
        FileSignature::Encrypt => {
            println!("encryption mode");
            let (e, n) = read_keys(file)?;

            println!("read e: {:?}", e);
            println!("read n: {:?}", n);

            return Ok(ReadBuf::Encrypt { e, n });
        },
        FileSignature::Decrypt => {
            println!("decryption mode");
            let (n, d) = read_keys(file)?;

            println!("read n: {:?}", n);
            println!("read d: {:?}", d);

            return Ok(ReadBuf::Decrypt { n, d });
        },
        FileSignature::Bin => {
            let mut buffer: Vec<u8> = vec![];
            file.read_to_end(&mut buffer)?;
            let buf: BigUint = BigUint::from_bytes_le(&buffer);
            return Ok(ReadBuf::Bin { buf });
        }
    }
}

fn read_keys(file: &mut std::fs::File) -> Result<(BigUint, BigUint), Box<dyn std::error::Error>> {
    // read in 8 bytes to fill two u32 values containing the sizes
    let mut size_buf1: [u8; 4]= Default::default();
    let mut size_buf2: [u8; 4]= Default::default();

    file.read_exact_at(&mut size_buf1, 2)?;
    file.read_exact_at(&mut size_buf2, 6)?;

    let size1: u32 = u32::from_le_bytes(size_buf1);
    let mut buf1: Vec<u8> = vec![0u8; size1.try_into()?];
    file.read_exact_at(&mut buf1, 10)?;
    let key1: BigUint = BigUint::from_le_bytes(&buf1);

    let size2: u32 = u32::from_le_bytes(size_buf2);
    let mut buf2: Vec<u8> = vec![0u8; size2.try_into()?];
    file.read_exact_at(&mut buf2, (9 + size1).try_into()?)?;
    let key2: BigUint = BigUint::from_bytes_le(&buf2);

    Ok((key1, key2))
}
