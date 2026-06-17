use clap::{Args, Parser, Subcommand};
use serde_json::json;
use soroban_toolkit::{
    address::{detect_address_type, mask_address, validate_address, AddressType},
    encoding::{from_base64, from_hex, to_base64, to_hex},
    hash::{double_sha256, sha256_hex, sha512_hex},
    transaction::{estimate_fee, format_xlm, is_valid_tx_hash, normalize_tx_hash},
};
use std::process;

#[derive(Parser)]
#[command(name = "soroban-toolkit", about = "Soroban utility toolkit", version)]
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
    Address(AddressArgs),
    /// Hash utilities
    Hash(HashArgs),
    /// Encoding utilities
    Encode(EncodeArgs),
    /// Transaction utilities
    Tx(TxArgs),
}

#[derive(Args)]
struct AddressArgs {
    /// Output results as JSON
    #[arg(long)]
    json: bool,
    #[command(subcommand)]
    action: AddressCommands,
}

#[derive(Args)]
struct HashArgs {
    /// Output results as JSON
    #[arg(long)]
    json: bool,
    #[command(subcommand)]
    action: HashCommands,
}

#[derive(Args)]
struct EncodeArgs {
    /// Output results as JSON
    #[arg(long)]
    json: bool,
    #[command(subcommand)]
    action: EncodeCommands,
}

#[derive(Args)]
struct TxArgs {
    /// Output results as JSON
    #[arg(long)]
    json: bool,
    #[command(subcommand)]
    action: TxCommands,
}

#[derive(Subcommand)]
enum AddressCommands {
    /// Validates a Stellar/Soroban address
    Validate { address: String },
    /// Masks a Stellar/Soroban address
    Mask { address: String },
    /// Detects the type of a Stellar/Soroban address
    Detect { address: String },
    /// Detects the type of a Stellar/Soroban address
    DetectType { address: String },
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

fn ok_json(data: serde_json::Value) -> String {
    json!({"success": true, "data": data}).to_string()
}

fn err_json(msg: &str) -> String {
    json!({"success": false, "error": msg}).to_string()
}

fn main() {
    let cli = Cli::parse();
    let global_json = cli.json;

    match cli.command {
        Commands::Address(a) => run_address(a.action, global_json || a.json),
        Commands::Hash(a) => run_hash(a.action, global_json || a.json),
        Commands::Encode(a) => run_encode(a.action, global_json || a.json),
        Commands::Tx(a) => run_tx(a.action, global_json || a.json),
    }
}

fn run_address(action: AddressCommands, use_json: bool) {
    match action {
        AddressCommands::Validate { address } => match validate_address(&address) {
            Ok(_) => {
                if use_json {
                    println!("{}", ok_json(json!({"valid": true, "address": address})));
                } else {
                    println!("Address is valid: {}", address);
                }
            }
            Err(e) => {
                if use_json {
                    println!("{}", err_json(&e.to_string()));
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
                    println!("{}", ok_json(json!(masked)));
                } else {
                    println!("{}", masked);
                }
            }
            Err(e) => {
                if use_json {
                    println!("{}", err_json(&e.to_string()));
                } else {
                    eprintln!("Error: {}", e);
                }
                process::exit(1);
            }
        },
        AddressCommands::Detect { address } | AddressCommands::DetectType { address } => {
            let addr_type = detect_address_type(&address);
            let type_str = match addr_type {
                AddressType::Account => "Account",
                AddressType::Contract => "Contract",
                AddressType::Invalid => "Invalid",
            };
            if use_json {
                println!("{}", ok_json(json!(type_str)));
            } else {
                println!("{}", type_str);
            }
        }
    }
}

fn run_hash(action: HashCommands, use_json: bool) {
    let digest = match action {
        HashCommands::Sha256 { input } => sha256_hex(input.as_bytes()),
        HashCommands::Sha512 { input } => sha512_hex(input.as_bytes()),
        HashCommands::DoubleSha256 { input } => double_sha256(input.as_bytes()),
    };
    if use_json {
        println!("{}", ok_json(json!(digest)));
    } else {
        println!("{}", digest);
    }
}

fn run_encode(action: EncodeCommands, use_json: bool) {
    match action {
        EncodeCommands::ToHex { input } => {
            let result = to_hex(input.as_bytes());
            if use_json {
                println!("{}", ok_json(json!(result)));
            } else {
                println!("{}", result);
            }
        }
        EncodeCommands::FromHex { input } => match from_hex(&input) {
            Ok(bytes) => {
                let result = String::from_utf8_lossy(&bytes).into_owned();
                if use_json {
                    println!("{}", ok_json(json!(result)));
                } else {
                    println!("{}", result);
                }
            }
            Err(e) => {
                if use_json {
                    println!("{}", err_json(&e.to_string()));
                } else {
                    eprintln!("Error: {}", e);
                }
                process::exit(1);
            }
        },
        EncodeCommands::ToBase64 { input } => {
            let result = to_base64(input.as_bytes());
            if use_json {
                println!("{}", ok_json(json!(result)));
            } else {
                println!("{}", result);
            }
        }
        EncodeCommands::FromBase64 { input } => match from_base64(&input) {
            Ok(bytes) => {
                let result = String::from_utf8_lossy(&bytes).into_owned();
                if use_json {
                    println!("{}", ok_json(json!(result)));
                } else {
                    println!("{}", result);
                }
            }
            Err(e) => {
                if use_json {
                    println!("{}", err_json(&e.to_string()));
                } else {
                    eprintln!("Error: {}", e);
                }
                process::exit(1);
            }
        },
    }
}

fn run_tx(action: TxCommands, use_json: bool) {
    match action {
        TxCommands::FormatXlm { stroops } => {
            let result = format_xlm(stroops);
            if use_json {
                println!("{}", ok_json(json!(result)));
            } else {
                println!("{}", result);
            }
        }
        TxCommands::ValidateHash { hash } => {
            if is_valid_tx_hash(&hash) {
                if use_json {
                    println!("{}", ok_json(json!({"valid": true, "hash": hash})));
                } else {
                    println!("valid");
                }
            } else {
                if use_json {
                    println!("{}", err_json("Invalid transaction hash"));
                } else {
                    eprintln!("Error: Invalid transaction hash");
                }
                process::exit(1);
            }
        }
        TxCommands::NormalizeHash { hash } => match normalize_tx_hash(&hash) {
            Ok(normalized) => {
                if use_json {
                    println!("{}", ok_json(json!(normalized)));
                } else {
                    println!("{}", normalized);
                }
            }
            Err(e) => {
                if use_json {
                    println!("{}", err_json(&e.to_string()));
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
            let xlm = format_xlm(stroops as u64);
            if use_json {
                println!("{}", ok_json(json!({"stroops": stroops, "xlm": xlm})));
            } else {
                println!("{} stroops ({})", stroops, xlm);
            }
        }
    }
}
