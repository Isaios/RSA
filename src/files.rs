use std::{fs::File, io::{Write, Read}, os::unix::prelude::FileExt, vec};
use num_bigint::BigUint;
use num_traits::FromBytes;

pub enum FileSignature {
    Encrypt,
    Decrypt,
    Bin,
    None,
}

// file format: 
// first two bytes evaluate to the ascii values either representing e and n or n and d, indicating
// what file type is beeing read. After that the lengths in bytes of the two values follows, so
// that they can be read in properly
//
//   0   1   2   3   4   5   6   7   8   9
// |___|___|___|___|___|___|___|___|___|___|___->
//  id1 id2 <-----len1----> <----len2-----> bytes
pub fn read_bufsig(path: std::path::PathBuf) -> Result<FileSignature, Box<dyn std::error::Error>>{
    let ref mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return Ok(FileSignature::None);
        },
    };
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

pub enum FileBuf {
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

pub enum ReadMode {
    Normal,
    Binary,
}

pub fn read_buf(path: std::path::PathBuf) -> Result<FileBuf, Box<dyn std::error::Error>> {
    let sig: FileSignature = read_bufsig(path.clone())?;
    match sig {
        FileSignature::Encrypt => {
            let ref mut file = File::open(path.clone())?;
            let (e, n) = read_keys(file)?;

            return Ok(FileBuf::Encrypt { e, n });
        },
        FileSignature::Decrypt => {
            let ref mut file = File::open(path.clone())?;
            let (n, d) = read_keys(file)?;

            return Ok(FileBuf::Decrypt { n, d });
        },
        FileSignature::Bin => {
            let ref mut file = File::open(path.clone())?;
            let mut buffer: Vec<u8> = vec![];
            file.read_to_end(&mut buffer)?;
            let buf: BigUint = BigUint::from_bytes_le(&buffer);
            return Ok(FileBuf::Bin { buf });
        },
        FileSignature::None => {
            return Err(format!("No such file or directory: {:?}", path).into());
        }
    }
}

pub fn read_bin_buf(path: std::path::PathBuf) -> Result<BigUint, Box<dyn std::error::Error>> {
    let sig: FileSignature = read_bufsig(path.clone())?; 
    match sig {
        FileSignature::None => {
            return Err(format!("No such file or directory: {:?}", path).into());
        }
        _ => {
            let ref mut file = File::open(path.clone())?;
            let mut buffer: Vec<u8> = vec![];
            file.read_to_end(&mut buffer)?;
            let buf: BigUint = BigUint::from_bytes_le(&buffer);
            return Ok(buf);
        }
    }
}

fn read_keys(file: &mut std::fs::File) -> Result<(BigUint, BigUint), Box<dyn std::error::Error>> {
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
    file.read_exact_at(&mut buf2, (10 + size1).try_into()?)?;
    let key2: BigUint = BigUint::from_bytes_le(&buf2);

    Ok((key1, key2))
}

pub fn write_buf(file: &mut std::fs::File, file_buf: FileBuf) -> Result<(), Box<dyn std::error::Error>> {
    match file_buf {
        FileBuf::Encrypt { e, n } => {
            let mut id_buf: Vec<u8> = vec![101, 110];
            let mut buf1 = e.to_bytes_le();
            let mut buf2 = n.to_bytes_le();

            write_keys(file, &mut id_buf, &mut buf1, &mut buf2)?;
            Ok(())
        },
        FileBuf::Decrypt { n, d } => {
            let mut id_buf: Vec<u8> = vec![110, 100];
            let mut buf1 = n.to_bytes_le();
            let mut buf2 = d.to_bytes_le();

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

pub fn write_bin_buf(num: BigUint, path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = num.to_bytes_le();
    let mut file = File::create(path)?;
    file.write_all(&mut buf)?;
    Ok(())
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
