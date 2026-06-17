use clap::{Parser, Subcommand};
use serde_json::json;
use soroban_toolkit::address::{detect_address_type, mask_address, validate_address, AddressType};
use soroban_toolkit::encoding::{from_base64, from_hex, to_base64, to_hex};
use soroban_toolkit::hash::{double_sha256, sha256_hex, sha512_hex};
use soroban_toolkit::transaction::{
    estimate_fee, format_xlm, is_valid_tx_hash, normalize_tx_hash, stroops_to_xlm,
};
use std::process;

#[derive(Parser)]
#[command(name = "soroban-toolkit")]
#[command(about = "Soroban utility toolkit", version)]
struct Cli {
    /// Output results as JSON
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
    /// Hashing utilities
    Hash {
        #[command(subcommand)]
        action: HashCommands,
    },
    /// Encoding/decoding utilities
    Encode {
        #[command(subcommand)]
        action: EncodeCommands,
    },
    /// Transaction utilities
    Tx {
        #[command(subcommand)]
        action: TxCommands,
    },
}

#[derive(Subcommand)]
enum AddressCommands {
    /// Validates a Stellar/Soroban address
    Validate {
        /// The Stellar address to validate
        address: String,
    },
    /// Masks a Stellar/Soroban address showing only first 4 and last 4 characters
    Mask {
        /// The Stellar address to mask
        address: String,
    },
    /// Detects the type of a Stellar/Soroban address (Account or Contract)
    #[command(name = "detect-type")]
    DetectType {
        /// The Stellar address to detect
        address: String,
    },
}

#[derive(Subcommand)]
enum HashCommands {
    /// Compute SHA-256 hash
    Sha256 {
        /// Input string to hash
        input: String,
    },
    /// Compute SHA-512 hash
    Sha512 {
        /// Input string to hash
        input: String,
    },
    /// Compute double SHA-256 hash
    #[command(name = "double-sha256")]
    DoubleSha256 {
        /// Input string to hash
        input: String,
    },
}

#[derive(Subcommand)]
enum EncodeCommands {
    /// Encode string to hex
    #[command(name = "to-hex")]
    ToHex {
        /// Input string to encode
        input: String,
    },
    /// Decode hex to string
    #[command(name = "from-hex")]
    FromHex {
        /// Hex string to decode
        input: String,
    },
    /// Encode string to base64
    #[command(name = "to-base64")]
    ToBase64 {
        /// Input string to encode
        input: String,
    },
    /// Decode base64 to string
    #[command(name = "from-base64")]
    FromBase64 {
        /// Base64 string to decode
        input: String,
    },
}

#[derive(Subcommand)]
enum TxCommands {
    /// Format stroops as XLM string
    #[command(name = "format-xlm")]
    FormatXlm {
        /// Stroops amount
        stroops: u64,
    },
    /// Validate a transaction hash
    #[command(name = "validate-hash")]
    ValidateHash {
        /// Transaction hash to validate
        hash: String,
    },
    /// Normalize a transaction hash (lowercase, strip 0x)
    #[command(name = "normalize-hash")]
    NormalizeHash {
        /// Transaction hash to normalize
        hash: String,
    },
    /// Estimate transaction fee
    #[command(name = "estimate-fee")]
    EstimateFee {
        /// Base fee in stroops
        base_fee: u32,
        /// Number of operations
        operations: u32,
    },
}

fn ok_json(data: serde_json::Value) {
    println!("{}", json!({"success": true, "data": data}));
}

fn err_json(msg: &str) {
    println!("{}", json!({"success": false, "error": msg}));
}

fn main() {
    let cli = Cli::parse();
    let use_json = cli.json;

    match cli.command {
        Commands::Address { action } => match action {
            AddressCommands::Validate { address } => match validate_address(&address) {
                Ok(_) => {
                    if use_json {
                        ok_json(json!({"valid": true, "address": address}));
                    } else {
                        println!("Address is valid: {}", address);
                    }
                }
                Err(e) => {
                    if use_json {
                        err_json(&e.to_string());
                    } else {
                        eprintln!("Error: {}", e);
                    }
                    process::exit(1);
                }
            },
            AddressCommands::Mask { address } => match validate_address(&address) {
                Ok(_) => {
                    let masked = mask_address(&address);
                    if use_json {
                        ok_json(json!(masked));
                    } else {
                        println!("{}", masked);
                    }
                }
                Err(e) => {
                    if use_json {
                        err_json(&e.to_string());
                    } else {
                        eprintln!("Error: {}", e);
                    }
                    process::exit(1);
                }
            },
            AddressCommands::DetectType { address } => {
                let addr_type = match detect_address_type(&address) {
                    AddressType::Account => "Account",
                    AddressType::Contract => "Contract",
                    AddressType::Invalid => "Invalid",
                };
                if use_json {
                    ok_json(json!(addr_type));
                } else {
                    println!("{}", addr_type);
                }
            }
        },

        Commands::Hash { action } => match action {
            HashCommands::Sha256 { input } => {
                let digest = sha256_hex(input.as_bytes());
                if use_json {
                    ok_json(json!(digest));
                } else {
                    println!("{}", digest);
                }
            }
            HashCommands::Sha512 { input } => {
                let digest = sha512_hex(input.as_bytes());
                if use_json {
                    ok_json(json!(digest));
                } else {
                    println!("{}", digest);
                }
            }
            HashCommands::DoubleSha256 { input } => {
                let digest = double_sha256(input.as_bytes());
                if use_json {
                    ok_json(json!(digest));
                } else {
                    println!("{}", digest);
                }
            }
        },

        Commands::Encode { action } => match action {
            EncodeCommands::ToHex { input } => {
                let encoded = to_hex(input.as_bytes());
                if use_json {
                    ok_json(json!(encoded));
                } else {
                    println!("{}", encoded);
                }
            }
            EncodeCommands::FromHex { input } => match from_hex(&input) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => {
                        if use_json {
                            ok_json(json!(s));
                        } else {
                            println!("{}", s);
                        }
                    }
                    Err(_) => {
                        let msg = "Decoded bytes are not valid UTF-8";
                        if use_json {
                            err_json(msg);
                        } else {
                            eprintln!("Error: {}", msg);
                        }
                        process::exit(1);
                    }
                },
                Err(e) => {
                    if use_json {
                        err_json(&e.to_string());
                    } else {
                        eprintln!("Error: {}", e);
                    }
                    process::exit(1);
                }
            },
            EncodeCommands::ToBase64 { input } => {
                let encoded = to_base64(input.as_bytes());
                if use_json {
                    ok_json(json!(encoded));
                } else {
                    println!("{}", encoded);
                }
            }
            EncodeCommands::FromBase64 { input } => match from_base64(&input) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => {
                        if use_json {
                            ok_json(json!(s));
                        } else {
                            println!("{}", s);
                        }
                    }
                    Err(_) => {
                        let msg = "Decoded bytes are not valid UTF-8";
                        if use_json {
                            err_json(msg);
                        } else {
                            eprintln!("Error: {}", msg);
                        }
                        process::exit(1);
                    }
                },
                Err(e) => {
                    if use_json {
                        err_json(&e.to_string());
                    } else {
                        eprintln!("Error: {}", e);
                    }
                    process::exit(1);
                }
            },
        },

        Commands::Tx { action } => match action {
            TxCommands::FormatXlm { stroops } => {
                let formatted = format_xlm(stroops);
                if use_json {
                    ok_json(json!(formatted));
                } else {
                    println!("{}", formatted);
                }
            }
            TxCommands::ValidateHash { hash } => {
                if is_valid_tx_hash(&hash) {
                    if use_json {
                        ok_json(json!({"valid": true}));
                    } else {
                        println!("valid");
                    }
                } else {
                    if use_json {
                        err_json("Invalid transaction hash");
                    } else {
                        eprintln!("Error: Invalid transaction hash");
                    }
                    process::exit(1);
                }
            }
            TxCommands::NormalizeHash { hash } => match normalize_tx_hash(&hash) {
                Ok(normalized) => {
                    if use_json {
                        ok_json(json!(normalized));
                    } else {
                        println!("{}", normalized);
                    }
                }
                Err(e) => {
                    if use_json {
                        err_json(&e.to_string());
                    } else {
                        eprintln!("Error: {}", e);
                    }
                    process::exit(1);
                }
            },
            TxCommands::EstimateFee {
                base_fee,
                operations,
            } => {
                let stroops = estimate_fee(base_fee, operations);
                let xlm = format!("{:.7} XLM", stroops_to_xlm(stroops as u64));
                if use_json {
                    ok_json(json!({"stroops": stroops, "xlm": xlm}));
                } else {
                    println!("{}", xlm);
                }
            }
        },
    }
}
