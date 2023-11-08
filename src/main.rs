mod primagen;
mod rsa;
mod parse;

use std::{fs::File, io::{Write, Read}, os::unix::prelude::FileExt, vec};

use num_bigint::{BigUint, ToBigUint};
use num_traits::{ToPrimitive, FromBytes};
use primagen::generate_primes;
use rsa::create_keys;

fn main(){
    // parse::parse();
    /* {
        // e: 101 | n: 110 | d: 100
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
    } */

    /* let n: BigUint = 258.to_biguint().unwrap();
    let e: BigUint = 65538.to_biguint().unwrap(); */

    /* let mut e_buf = e.to_bytes_le();
    let mut n_buf = n.to_bytes_le();
    println!("{:?}", e_buf);
    println!("{:?}", n_buf);

    let mut write_buf: Vec<u8> = vec![];

    // write_buf.append(&mut e_buf);
    write_buf.append(&mut n_buf);

    let mut file = File::create("test").unwrap();
    file.write_all(&mut write_buf).unwrap();

    {
        let mut file = File::open("test").unwrap();
        let mut read_buf: Vec<u8> = vec![];
        file.read_to_end(&mut read_buf).unwrap();
        println!("read: {:?}", read_buf);
        println!("{:?}", BigUint::from_bytes_le(&mut read_buf));
    } */

    /* let mut file = File::create("test").unwrap();
    write_buf(&mut file, FileBuf::Encrypt { e, n }).unwrap();

    // let sig: FileSignature = read_bufsig("test".into()).unwrap();
    read_buf("test".into()).unwrap(); */
    /* {
        let mut file = File::open("test").unwrap();
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf).unwrap();
        for byte in buf {
            println!("{byte}");
        }
    } */
    let (e, n, d) = create_keys(generate_primes(1024, 500));
    println!("e: {:?}", e);
    println!("n: {:?}", n);
    println!("d: {:?}", d);

    let mut pub_file = File::create("pub.key").unwrap();
    let mut priv_file = File::create("priv.key").unwrap();

    write_buf(&mut pub_file, FileBuf::Encrypt { e, n: n.clone() }).unwrap();
    write_buf(&mut priv_file, FileBuf::Decrypt { n, d }).unwrap();
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

enum FileBuf {
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

fn read_buf(path: std::path::PathBuf) -> Result<FileBuf, Box<dyn std::error::Error>> {
    let sig: FileSignature = read_bufsig(path.clone()).unwrap();
    let ref mut file = File::open(path)?;
    match sig {
        FileSignature::Encrypt => {
            let (e, n) = read_keys(file)?;

            /* println!("encryption mode");
            println!("read e: {:?}", e);
            println!("read n: {:?}", n); */

            return Ok(FileBuf::Encrypt { e, n });
        },
        FileSignature::Decrypt => {
            let (n, d) = read_keys(file)?;

            /* println!("decryption mode");
            println!("read n: {:?}", n);
            println!("read d: {:?}", d); */

            return Ok(FileBuf::Decrypt { n, d });
        },
        FileSignature::Bin => {
            let mut buffer: Vec<u8> = vec![];
            file.read_to_end(&mut buffer)?;
            let buf: BigUint = BigUint::from_bytes_le(&buffer);
            return Ok(FileBuf::Bin { buf });
        }
    }
}

fn read_keys(file: &mut std::fs::File) -> Result<(BigUint, BigUint), Box<dyn std::error::Error>> {
    // read in 8 bytes to fill two u32 values containing the sizes
    let mut size_buf1: [u8; 4]= Default::default();
    let mut size_buf2: [u8; 4]= Default::default();

    {
        let mut read_buf: Vec<u8> = vec![];
        file.read_to_end(&mut read_buf)?;
        println!("{:?}", read_buf);
    }

    file.read_exact_at(&mut size_buf1, 2)?;
    file.read_exact_at(&mut size_buf2, 6)?;

    let size1: u32 = u32::from_le_bytes(size_buf1);
    let mut buf1: Vec<u8> = vec![0u8; size1.try_into()?];
    file.read_exact_at(&mut buf1, 10)?;
    let key1: BigUint = BigUint::from_le_bytes(&buf1);

    let size2: u32 = u32::from_le_bytes(size_buf2);
    let mut buf2: Vec<u8> = vec![0u8; size2.try_into()?];
    file.read_exact_at(&mut buf2, (10 + size1).try_into()?)?;
    let key2: BigUint = BigUint::from_bytes_le(&buf2);

    Ok((key1, key2))
}

fn write_buf(file: &mut std::fs::File, file_buf: FileBuf) -> Result<(), Box<dyn std::error::Error>> {
    match file_buf {
        FileBuf::Encrypt { e, n } => {
            let mut id_buf: Vec<u8> = vec![101, 110];
            let mut buf1 = e.to_bytes_le();
            let mut buf2 = n.to_bytes_le();
            /* println!("{:?}", buf1);
            println!("{:?}", buf2); */
            write_keys(file, &mut id_buf, &mut buf1, &mut buf2)?;
            Ok(())
        },
        FileBuf::Decrypt { n, d } => {
            let mut id_buf: Vec<u8> = vec![110, 100];
            let mut buf1 = n.to_bytes_le();
            let mut buf2 = d.to_bytes_le();
            /* println!("{:?}", buf1);
            println!("{:?}", buf2); */
            write_keys(file, &mut id_buf, &mut buf1, &mut buf2)?;
            Ok(())
        },
        FileBuf::Bin { buf } => {
            let mut buf: Vec<u8> = buf.to_bytes_le();
            file.write_all(&mut buf)?;
            Ok(())
        }
    }
}

fn write_keys(file: &mut std::fs::File, id_buf: &mut Vec<u8>, buf1: &mut Vec<u8>, buf2: &mut Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer: Vec<u8> = vec![];
    buffer.append(id_buf);

    let len1: u32 = buf1.len().try_into()?;
    let len2: u32 = buf2.len().try_into()?;

    buffer.append(&mut len1.to_le_bytes().to_vec());
    buffer.append(&mut len2.to_le_bytes().to_vec());
    buffer.append(buf1);
    buffer.append(buf2);

    file.write_all(&buffer)?;
    Ok(())
}
