use clap::{Args, Parser, Subcommand};
use num_bigint::BigUint;
use std::fs::File;

use crate::files::*;
use crate::primagen::*;
use crate::rsa::*;

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_command: SubCommand,
}

#[allow(non_camel_case_types)]
#[derive(Subcommand)]
enum SubCommand {
    Generate {
        #[arg(short = 'l', long, default_value_t = 1024)]
        key_length: u16,
        #[arg(long, value_name = "PUBLIC_KEY_FILE", requires("private"))]
        public: Option<std::path::PathBuf>,
        #[arg(long, value_name = "PIVATE_KEY_FILE", requires("public"))]
        private: Option<std::path::PathBuf>,
    },
    Encrypt {
        #[command(flatten)]
        input: InputArgs,
        #[command(flatten)]
        enc_keys: EncKeyArgs,
        #[arg(short, long)]
        out_file: Option<std::path::PathBuf>,
    },
    Decrypt {
        #[command(flatten)]
        input: InputArgs,
        #[command(flatten)]
        dec_keys: DecKeyArgs,
        #[arg(short, long)]
        out_file: Option<std::path::PathBuf>,
    },
    Primes {
        #[arg(short, long)]
        count: usize,
        #[arg(short, long, default_value_t = 1024)]
        length: usize,
    },
    Write_Public {
        #[arg(short, long)]
        file: std::path::PathBuf,
        #[arg(short, value_name = "ENCRYPT_EXPONENT")]
        e: BigUint,
        #[arg(short, value_name = "ENCRYPT_MODULUS")]
        n: BigUint,
    },
    Write_Private {
        #[arg(short, long)]
        file: std::path::PathBuf,
        #[arg(short, value_name = "DECRYPT_MODULUS")]
        n: BigUint,
        #[arg(short, value_name = "DECRYPT_EXPONENT")]
        d: BigUint,
    },
    Read {
        file: std::path::PathBuf,
    }
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct InputArgs {
    #[arg(short = 'f', long)]
    input_file: Option<std::path::PathBuf>,
    #[arg(short = 'i', long)]
    input: Option<BigUint>,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
struct EncKeyArgs {
    #[clap(conflicts_with("n"), conflicts_with("d"))]
    #[arg(short = 'k', long)]
    key_file: Option<std::path::PathBuf>,

    #[arg(short, requires("n"), value_name = "ENCRYPT_EXPONENT")]
    e: Option<BigUint>,
    #[arg(short, requires("e"), value_name = "ENCRYPT_MODULUS")]
    n: Option<BigUint>,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
struct DecKeyArgs {
    #[clap(conflicts_with("n"), conflicts_with("e"))]
    #[arg(short = 'k', long)]
    key_file: Option<std::path::PathBuf>,

    #[arg(short, requires("d"), value_name = "DECRYPT_MODULUS")]
    n: Option<BigUint>,
    #[arg(short, requires("n"), value_name = "DECTYPT_EXPONENT")]
    d: Option<BigUint>,
}

pub fn parse() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    match args.sub_command {
        SubCommand::Generate {
            key_length,
            public,
            private,
        } => {
            // generate the keys using the length
            let (e, n, d) = create_keys(generate_primes(key_length.into(), 500));
            if public.is_some() && private.is_some() {
                // write generated keys to public and private key files
                match read_bufsig(public.clone().expect(""))? {
                    FileSignature::None => {}
                    _ => {
                        if !user_confirmation(
                            format!("{:?} already exist. Overwrite?", &public),
                            ConfirmOpts::No,
                        ) {
                            return Ok(());
                        }
                    }
                };
                match read_bufsig(private.clone().expect(""))? {
                    FileSignature::None => {}
                    _ => {
                        if !user_confirmation(
                            format!("{:?} already exist. Overwrite?", &private),
                            ConfirmOpts::No,
                        ) {
                            return Ok(());
                        }
                    }
                }
                let mut pub_file = File::create(public.clone().expect(""))?;
                write_buf(&mut pub_file, FileBuf::Encrypt { e, n: n.clone() })?;
                let mut priv_file = File::create(private.clone().expect(""))?;
                write_buf(&mut priv_file, FileBuf::Decrypt { d, n })?;
            } else if public.is_some() || private.is_some() {
                return Err("Both key files need to be provided".into());
            } else {
                println!("e: {:?}\nn: {:?}\nd: {:?}", e, n, d);
            }
        }
        SubCommand::Encrypt {
            input,
            enc_keys,
            out_file,
        } => {
            let encrypted: BigUint;
            let (e, n) = get_enc_keys(enc_keys)?;
            let z: BigUint;
            match input.input_file {
                Some(input_file) => {
                    // read input from the input_file and procced with encryption
                    z = read_bin_buf(input_file)?;
                }
                None => match input.input {
                    Some(input) => {
                        // use input and keys to encrypt
                        z = input;
                    }
                    None => return Err("No input provided for encryption".into()),
                },
            };
            encrypted = rsa_encrypt(&z, &e, &n);
            match out_file {
                Some(file) => {
                    // write output to file
                    match read_bufsig(file.clone())? {
                        FileSignature::None => {}
                        _ => {
                            if !user_confirmation(
                                format!("{:?} does already exist. Overwrite?", file),
                                ConfirmOpts::Yes,
                            ) {
                                return Ok(());
                            }
                        }
                    }
                    write_bin_buf(encrypted, file)?;
                }
                None => {
                    // print output to standard out
                    println!("enrypted: {:?}", encrypted);
                }
            };
        }
        SubCommand::Decrypt {
            input,
            dec_keys,
            out_file,
        } => {
            let decrypted: BigUint;
            let (n, d) = get_dec_keys(dec_keys)?;
            let c: BigUint;
            match input.input_file {
                Some(input_file) => {
                    c = read_bin_buf(input_file)?;
                }
                None => match input.input {
                    Some(input) => {
                        c = input;
                    }
                    None => {
                        return Err("No input for decrytption provided".into());
                    }
                },
            };
            decrypted = rsa_decrypt(&c, &d, &n);
            match out_file {
                Some(file) => {
                    // write output to file
                    match read_bufsig(file.clone())? {
                        FileSignature::None => {}
                        _ => {
                            if !user_confirmation(
                                format!("{:?} does already exist. Overwrite?", file),
                                ConfirmOpts::Yes,
                            ) {
                                return Ok(());
                            }
                        }
                    }
                    write_bin_buf(decrypted, file)?;
                }
                None => {
                    // print output to standard out
                    println!("decrypted: {:?}", decrypted);
                }
            };
        },
        SubCommand::Primes { count, length } => {
            // generate <count> primes and print them to the standard out
            let start = std::time::Instant::now();
            gen_primes(length, 500, count);
            println!("time to generate: {:?}", start.elapsed());
        },
        SubCommand::Write_Public { file, e, n } => {
            match read_bufsig(file.clone())? {
                FileSignature::None => {},
                _ => {
                    if !user_confirmation(format!("{:?} does already exist. Overwrite?", file), ConfirmOpts::No) {
                        return Ok(());
                    }
                }
            };
            let mut key_file = File::create(file)?;
            write_buf(&mut key_file, FileBuf::Encrypt { e, n })?;
        },
        SubCommand::Write_Private { file, n, d } => {
            match read_bufsig(file.clone())? {
                FileSignature::None => {},
                _ => {
                    if !user_confirmation(format!("{:?} does already exist. Overwrite?", file), ConfirmOpts::No) {
                        return Ok(());
                    }
                }
            };
            let mut key_file = File::create(file)?;
            write_buf(&mut key_file, FileBuf::Decrypt { n, d })?;
        },
        SubCommand::Read { file } => {
            let read_buf = read_buf(file)?;
            match read_buf {
                FileBuf::Encrypt { e, n } => {
                    println!("e: {:?}\nn: {:?}", e, n);
                },
                FileBuf::Decrypt { n, d } => {
                    println!("n: {:?}\nd: {:?}", n, d);
                },
                FileBuf::Bin { buf: _ } => {
                    println!("No key file");
                },
            }
        }
    }
    Ok(())
}

/// function that extracts the e and n value from the EncKeyArgs
/// panics when one or both are missing
///
/// # Example
/// ```
/// let (e, n) = get_enc_keys(enc_key_args);
/// ```
fn get_enc_keys(
    enc_key_args: EncKeyArgs,
) -> Result<(BigUint, BigUint), Box<dyn std::error::Error>> {
    match enc_key_args.key_file {
        Some(key_file) => {
            // read in keys from file and return them
            let file_buf = read_buf(key_file)?;

            match file_buf {
                FileBuf::Encrypt { e, n } => {
                    return Ok((e, n));
                }
                _ => {
                    return Err("Wrong file format provided for encryption".into());
                }
            }
        }
        None => {
            return Ok((
                match enc_key_args.e {
                    Some(e) => e,
                    None => return Err("No encryption exponent provided".into()),
                },
                match enc_key_args.n {
                    Some(n) => n,
                    None => return Err("No encryption modulus provided".into()),
                },
            ));
        }
    };
}

/// Function that extracts the n and d value from the DecKeyArgs
/// panics when one or both are missing
///
/// # Example
/// ```
/// let (n, d) = get_dec_keys(dec_key_args);
/// ```
fn get_dec_keys(
    dec_key_args: DecKeyArgs,
) -> Result<(BigUint, BigUint), Box<dyn std::error::Error>> {
    match dec_key_args.key_file {
        Some(key_file) => {
            // read in keys from file and return them
            let file_buf = read_buf(key_file)?;
            match file_buf {
                FileBuf::Decrypt { n, d } => {
                    return Ok((n, d));
                }
                _ => {
                    return Err("Wrong file format provided for decryption".into());
                }
            }
        }
        None => Ok((
            match dec_key_args.n {
                Some(n) => n,
                None => return Err("No decryption modulus provided".into()),
            },
            match dec_key_args.d {
                Some(d) => d,
                None => return Err("No decryption exponent provied".into()),
            },
        )),
    }
}

enum ConfirmOpts {
    Yes,
    No,
}

fn user_confirmation(message: String, c_opts: ConfirmOpts) -> bool {
    let mut input = String::new();
    let stdin = std::io::stdin();
    match c_opts {
        ConfirmOpts::Yes => {
            println!("{message} [Y/n]");
            stdin
                .read_line(&mut input)
                .expect("Could not read user input");
            if input == "\n" || input == "y\n" || input == "Y\n" {
                return true;
            } else {
                return false;
            }
        }
        ConfirmOpts::No => {
            println!("{message} [y/N]");
            stdin
                .read_line(&mut input)
                .expect("Could not read user input");
            if input == "y\n" || input == "Y\n" {
                return true;
            } else {
                return false;
            }
        }
    };
}
