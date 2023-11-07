use clap::{Args, Parser, Subcommand};
use num_bigint::BigUint;

//  rsa -> generate -> <key_length> <oFile?> /
//      |
//      |
//      |-> encrypt -> <iFile | input> <Keys <e> <n> | pubKeyFile> <oFile?> /
//      |
//      |
//      |-> decrypt -> <iFile | input> <Keys <d> <n> | privKeyFile> <oFile?>
//      |
//      |
//      |-> primes

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    sub_command: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Generate {
        #[arg(short = 'l', long)]
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
        out_file: Option<std::path::PathBuf>
    },
    Decrypt {
        #[command(flatten)]
        input: InputArgs,
        #[command(flatten)]
        dec_keys: DecKeyArgs,
        #[arg(short, long)]
        out_file: Option<std::path::PathBuf>
    },
    Primes {
        count: u32,
        length: u32,
    },
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

pub fn parse() {
    let args = Arguments::parse();

    match args.sub_command {
        SubCommand::Generate { key_length, public, private } => {
            // generate the keys using the length



            match public {
                Some(public) => {
                    match private {
                        Some(private) => {
                            // write generated keys to public and private key files



                        },
                        None => panic!("no private key file provided"),
                    }
                },
                None => (),
            };
        },
        SubCommand::Encrypt { input, enc_keys, out_file } => {
            let encrypted: BigUint = Default::default();

            match input.input_file {
                Some(input_file) => {
                    // read input from the input_file and procced with encryption
                    let (e, n) = get_enc_keys(enc_keys);



                },
                None => match input.input {
                    Some(input) => {
                        let (e, n) = get_enc_keys(enc_keys);
                        // use input and keys to encrypt
                        


                    },
                    None => panic!("No input for encryption provided"),
                },
            };

            match out_file {
                Some(file) => {
                    // write output to file



                },
                None => {
                    // print output to standard out
                    println!("enrypted: {:?}", encrypted);
                }
            };
        },
        SubCommand::Decrypt { input, dec_keys, out_file } => {
            let decrypted: BigUint = Default::default();
            match input.input_file {
                Some(input_file) => {
                    let (d, n) = get_dec_keys(dec_keys);



                },
                None => match input.input {
                    Some(input) => {
                        let (d, n) = get_dec_keys(dec_keys);



                    },
                    None => {
                        panic!("No input for decrytption provided")
                    }
                }
            };

            match out_file {
                Some(file) => {
                    // write output to file



                },
                None => {
                    // print output to standard out
                    println!("enrypted: {:?}", decrypted);
                }
            };
        },
        SubCommand::Primes { count, length } => {
            // generate <count> primes and print them to the standard out
        }
    }
}

/// function that extracts the e and n value from the EncKeyArgs
/// panics when one or both are missing
///
/// # Example
/// ```
/// let (e, n) = get_enc_keys(enc_key_args);
/// ```
fn get_enc_keys(enc_key_args: EncKeyArgs) -> (BigUint, BigUint) {
    match enc_key_args.key_file {
        Some(key_file) => {
            // read in keys from file and return them



            (1u8.into(), 1u8.into()) 
        },
        None => {
            (
                match enc_key_args.e {
                    Some(e) => e,
                    None => panic!("No encryption exponent provied"),
                },
                match enc_key_args.n {
                    Some(n) => n,
                    None => panic!("No encryption modulus provided"),
                }
            )
        }
    }
}

/// function that extracts the n and d value from the DecKeyArgs
/// panics when one or both are missing
///
/// # Example
/// ```
/// let (n, d) = get_dec_keys(dec_key_args);
/// ```
fn get_dec_keys(dec_key_args: DecKeyArgs) -> (BigUint, BigUint) {
    match dec_key_args.key_file {
        Some(key_file) => {
            // read in keys from file and return them



            (1u8.into(), 1u8.into()) 
        },
        None => {
            (
                match dec_key_args.n {
                    Some(e) => e,
                    None => panic!("No decryption exponent provied"),
                },
                match dec_key_args.d {
                    Some(n) => n,
                    None => panic!("No decryption modulus provided"),
                }
            )
        }
    }
}
