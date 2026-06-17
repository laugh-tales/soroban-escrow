use clap::{Parser, Subcommand};
use soroban_toolkit::{
    address::{detect_address_type, mask_address, validate_address, AddressType},
    encoding::{from_base64, from_hex, to_base64, to_hex},
    hash::{blake3_hex, double_sha256, sha256_hex, sha512_hex},
    transaction::{
        estimate_fee, format_xlm, is_valid_tx_hash, normalize_tx_hash, stroops_to_xlm,
        xlm_to_stroops,
    },
};
use std::io::{self, Read};
use std::process;

#[derive(Parser)]
#[command(name = "soroban-toolkit", about = "Soroban utility toolkit", version)]
struct Cli {
    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Address utilities
    Address {
        #[command(subcommand)]
        action: AddressCommands,
    },
    /// Hash data: `hash sha256|sha512|blake3|double-sha256 <INPUT|->`
    /// Also: `hash <INPUT> --algo sha256|sha512|blake3|double-sha256`
    #[command(allow_external_subcommands = true)]
    Hash {
        #[command(subcommand)]
        action: Option<HashSubcommand>,
    },
    /// Encode data
    #[command(allow_external_subcommands = true)]
    Encode {
        #[command(subcommand)]
        action: Option<EncodeSubcommand>,
    },
    /// Decode data (flat style)
    Decode {
        input: String,
        #[arg(long)]
        format: String,
    },
    /// Transaction utilities
    Tx {
        #[command(subcommand)]
        action: TxCommands,
    },
    /// XLM conversion utilities
    Xlm {
        #[command(subcommand)]
        action: XlmCommands,
    },
    /// Validate a Stellar address (flat style)
    #[command(name = "validate-address")]
    ValidateAddress { address: String },
}

#[derive(Subcommand)]
enum AddressCommands {
    Validate {
        address: String,
    },
    Mask {
        address: String,
    },
    #[command(alias = "detect")]
    DetectType {
        address: String,
    },
}

#[derive(Subcommand)]
enum HashSubcommand {
    Sha256 {
        input: String,
    },
    Sha512 {
        input: String,
    },
    Blake3 {
        input: String,
    },
    DoubleSha256 {
        input: String,
    },
    /// Flat-style catch-all: `hash <INPUT> --algo sha256`
    #[command(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Subcommand)]
enum EncodeSubcommand {
    ToHex {
        input: String,
    },
    FromHex {
        input: String,
    },
    ToBase64 {
        input: String,
    },
    FromBase64 {
        input: String,
    },
    /// Flat-style catch-all: `encode <INPUT> --format hex`
    #[command(external_subcommand)]
    Other(Vec<String>),
}

#[derive(Subcommand)]
enum TxCommands {
    FormatXlm { stroops: u64 },
    ValidateHash { hash: String },
    NormalizeHash { hash: String },
    EstimateFee { base_fee: u32, operations: u32 },
}

#[derive(Subcommand)]
enum XlmCommands {
    ToXlm { stroops: u64 },
    ToStroops { xlm: f64 },
    Format { stroops: u64 },
}

fn read_input(s: &str) -> String {
    if s == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("failed to read stdin");
        // Strip only a single trailing newline (as piped input adds one)
        if buf.ends_with('\n') {
            buf.pop();
            if buf.ends_with('\r') {
                buf.pop();
            }
        }
        buf
    } else {
        s.to_owned()
    }
}

fn ok_json(data: serde_json::Value) -> String {
    serde_json::json!({ "success": true, "data": data }).to_string()
}

fn err_json(msg: &str) -> String {
    serde_json::json!({ "success": false, "error": msg }).to_string()
}

fn hash_with_algo(input: &str, algo: &str, json: bool) {
    let bytes = input.as_bytes();
    let digest = match algo {
        "sha256" => sha256_hex(bytes),
        "sha512" => sha512_hex(bytes),
        "blake3" => blake3_hex(bytes),
        "double-sha256" => double_sha256(bytes),
        other => {
            eprintln!("Error: Unknown algorithm: {other}");
            process::exit(1);
        }
    };
    if json {
        println!("{}", ok_json(serde_json::json!(digest)));
    } else {
        println!("{digest}");
    }
}

fn main() {
    let cli = Cli::parse();
    let json = cli.json;

    match cli.command {
        Commands::ValidateAddress { address } => match validate_address(&address) {
            Ok(_) => println!("Address is valid: {address}"),
            Err(e) => {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        },

        Commands::Address { action } => match action {
            AddressCommands::Validate { address } => match validate_address(&address) {
                Ok(_) => {
                    if json {
                        println!(
                            "{}",
                            ok_json(serde_json::json!({"valid": true, "address": address}))
                        );
                    } else {
                        println!("Address is valid: {address}");
                    }
                }
                Err(e) => {
                    if json {
                        println!("{}", err_json(&e.to_string()));
                    } else {
                        eprintln!("Error: {e}");
                    }
                    process::exit(1);
                }
            },
            AddressCommands::Mask { address } => match validate_address(&address) {
                Ok(_) => {
                    let masked = mask_address(&address);
                    if json {
                        println!("{}", ok_json(serde_json::json!(masked)));
                    } else {
                        println!("{masked}");
                    }
                }
                Err(e) => {
                    if json {
                        println!("{}", err_json(&e.to_string()));
                    } else {
                        eprintln!("Error: {e}");
                    }
                    process::exit(1);
                }
            },
            AddressCommands::DetectType { address } => {
                let kind = match detect_address_type(&address) {
                    AddressType::Account => "Account",
                    AddressType::Contract => "Contract",
                    AddressType::Invalid => "Invalid",
                };
                if json {
                    println!("{}", ok_json(serde_json::json!(kind)));
                } else {
                    println!("{kind}");
                }
            }
        },

        Commands::Hash { action } => match action {
            Some(HashSubcommand::Sha256 { input }) => {
                hash_with_algo(&read_input(&input), "sha256", json)
            }
            Some(HashSubcommand::Sha512 { input }) => {
                hash_with_algo(&read_input(&input), "sha512", json)
            }
            Some(HashSubcommand::Blake3 { input }) => {
                hash_with_algo(&read_input(&input), "blake3", json)
            }
            Some(HashSubcommand::DoubleSha256 { input }) => {
                hash_with_algo(&read_input(&input), "double-sha256", json)
            }
            Some(HashSubcommand::Other(args)) => {
                // Flat style: `hash <INPUT> --algo sha256`
                // args[0] is the input, rest may include --algo <algo>
                let mut input = args.first().map(|s| s.as_str()).unwrap_or("").to_owned();
                let mut algo = "sha256".to_owned();
                let mut i = 1;
                while i < args.len() {
                    if args[i] == "--algo" {
                        if let Some(a) = args.get(i + 1) {
                            algo = a.clone();
                            i += 2;
                            continue;
                        }
                    }
                    i += 1;
                }
                if input == "-" {
                    input = read_input("-");
                }
                hash_with_algo(&input, &algo, json);
            }
            None => {
                eprintln!("Error: specify a hash algorithm subcommand (sha256, sha512, blake3, double-sha256)");
                process::exit(1);
            }
        },

        Commands::Encode { action } => match action {
            Some(EncodeSubcommand::ToHex { input }) => {
                let out = to_hex(input.as_bytes());
                if json {
                    println!("{}", ok_json(serde_json::json!(out)));
                } else {
                    println!("{out}");
                }
            }
            Some(EncodeSubcommand::FromHex { input }) => match from_hex(&input) {
                Ok(b) => {
                    let s = String::from_utf8_lossy(&b).into_owned();
                    if json {
                        println!("{}", ok_json(serde_json::json!(s)));
                    } else {
                        println!("{s}");
                    }
                }
                Err(e) => {
                    if json {
                        println!("{}", err_json(&e.to_string()));
                    } else {
                        eprintln!("Error: {e}");
                    }
                    process::exit(1);
                }
            },
            Some(EncodeSubcommand::ToBase64 { input }) => {
                let out = to_base64(input.as_bytes());
                if json {
                    println!("{}", ok_json(serde_json::json!(out)));
                } else {
                    println!("{out}");
                }
            }
            Some(EncodeSubcommand::FromBase64 { input }) => match from_base64(&input) {
                Ok(b) => {
                    let s = String::from_utf8_lossy(&b).into_owned();
                    if json {
                        println!("{}", ok_json(serde_json::json!(s)));
                    } else {
                        println!("{s}");
                    }
                }
                Err(e) => {
                    if json {
                        println!("{}", err_json(&e.to_string()));
                    } else {
                        eprintln!("Error: {e}");
                    }
                    process::exit(1);
                }
            },
            Some(EncodeSubcommand::Other(args)) => {
                // Flat style: `encode <INPUT> --format hex`
                let raw = args.first().map(|s| s.as_str()).unwrap_or("");
                let mut fmt = "hex".to_owned();
                let mut i = 1;
                while i < args.len() {
                    if args[i] == "--format" {
                        if let Some(f) = args.get(i + 1) {
                            fmt = f.clone();
                            i += 2;
                            continue;
                        }
                    }
                    i += 1;
                }
                let out = match fmt.as_str() {
                    "hex" => to_hex(raw.as_bytes()),
                    "base64" => to_base64(raw.as_bytes()),
                    other => {
                        eprintln!("Error: Unknown format: {other}");
                        process::exit(1);
                    }
                };
                println!("{out}");
            }
            None => {
                eprintln!("Error: specify an encode subcommand");
                process::exit(1);
            }
        },

        Commands::Decode { input, format } => {
            let result = match format.as_str() {
                "hex" => from_hex(&input).map(|b| String::from_utf8_lossy(&b).into_owned()),
                "base64" => from_base64(&input).map(|b| String::from_utf8_lossy(&b).into_owned()),
                other => {
                    eprintln!("Error: Unknown format: {other}");
                    process::exit(1);
                }
            };
            match result {
                Ok(s) => println!("{s}"),
                Err(e) => {
                    eprintln!("Error: {e}");
                    process::exit(1);
                }
            }
        }

        Commands::Tx { action } => match action {
            TxCommands::FormatXlm { stroops } => {
                let out = format_xlm(stroops);
                if json {
                    println!("{}", ok_json(serde_json::json!(out)));
                } else {
                    println!("{out}");
                }
            }
            TxCommands::ValidateHash { hash } => {
                if is_valid_tx_hash(&hash) {
                    if json {
                        println!("{}", ok_json(serde_json::json!({"valid": true})));
                    } else {
                        println!("valid");
                    }
                } else {
                    if json {
                        println!("{}", err_json("Invalid transaction hash"));
                    } else {
                        eprintln!("Error: Invalid transaction hash");
                    }
                    process::exit(1);
                }
            }
            TxCommands::NormalizeHash { hash } => match normalize_tx_hash(&hash) {
                Ok(h) => {
                    if json {
                        println!("{}", ok_json(serde_json::json!(h)));
                    } else {
                        println!("{h}");
                    }
                }
                Err(e) => {
                    if json {
                        println!("{}", err_json(&e.to_string()));
                    } else {
                        eprintln!("Error: {e}");
                    }
                    process::exit(1);
                }
            },
            TxCommands::EstimateFee {
                base_fee,
                operations,
            } => {
                let stroops = estimate_fee(base_fee, operations);
                let xlm = format_xlm(stroops as u64);
                if json {
                    println!(
                        "{}",
                        ok_json(serde_json::json!({"stroops": stroops, "xlm": xlm}))
                    );
                } else {
                    println!("{stroops} stroops ({xlm})");
                }
            }
        },

        Commands::Xlm { action } => match action {
            XlmCommands::ToXlm { stroops } => {
                let xlm = stroops_to_xlm(stroops);
                let out = if xlm.fract() == 0.0 {
                    format!("{}", xlm as u64)
                } else {
                    format!("{xlm}")
                };
                println!("{out}");
            }
            XlmCommands::ToStroops { xlm } => {
                println!("{}", xlm_to_stroops(xlm));
            }
            XlmCommands::Format { stroops } => {
                println!("{}", format_xlm(stroops));
            }
        },
    }
}
