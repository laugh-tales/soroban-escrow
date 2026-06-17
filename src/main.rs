use clap::{Parser, Subcommand, Args};
use soroban_toolkit::address::{detect_address_type, mask_address, validate_address, AddressType};
use soroban_toolkit::encoding::{from_base64, from_hex, to_base64, to_hex};
use soroban_toolkit::hash::{double_sha256, sha256_hex, sha512_hex};
use soroban_toolkit::transaction::{estimate_fee, estimate_fee_xlm, format_xlm, is_valid_tx_hash, normalize_tx_hash};
use std::process;

#[derive(Parser)]
#[command(name = "soroban-toolkit")]
#[command(about = "Soroban utility toolkit", version)]
struct Cli {
    /// Output JSON instead of plain text
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Address utilities
    Address(AddressSub),
    /// Legacy: validate-address <address>
    ValidateAddress { address: String },
    /// Hash utilities
    Hash(HashSub),
    /// Encoding utilities
    Encode(EncodeSub),
    /// Legacy decode command: decode <input> --format <format>
    Decode { input: String, #[arg(long)] format: String },
    
    /// Transaction utilities
    Tx(TxSub),
    /// XLM helpers (legacy)
    Xlm { #[command(subcommand)] action: XlmCommands },
}

#[derive(Args)]
struct AddressSub {
    #[command(subcommand)]
    action: AddressCommands,
}

#[derive(Subcommand)]
enum AddressCommands {
    /// Validates a Stellar/Soroban address
    Validate { address: String },
    /// Masks a Stellar/Soroban address showing only first 4 and last 4 characters
    Mask { address: String },
    /// Detects the type of a Stellar/Soroban address (Account or Contract)
    #[command(alias = "detect")]
    DetectType { address: String },
}

#[derive(Args)]
struct HashSub {
    #[command(subcommand)]
    action: Option<HashCommands>,
    /// Legacy positional input for the hash
    input: Option<String>,
    /// Legacy algorithm flag: sha256, sha512, double-sha256
    #[arg(long)]
    algo: Option<String>,
}

#[derive(Subcommand)]
enum HashCommands {
    Sha256 { input: String },
    Sha512 { input: String },
    DoubleSha256 { input: String },
}

#[derive(Args)]
struct EncodeSub {
    #[command(subcommand)]
    action: Option<EncodeCommands>,
    /// Legacy positional input for encode/decode
    input: Option<String>,
    /// Legacy format flag: hex or base64
    #[arg(long)]
    format: Option<String>,
}

#[derive(Subcommand)]
enum EncodeCommands {
    ToHex { input: String },
    FromHex { input: String },
    ToBase64 { input: String },
    FromBase64 { input: String },
}

#[derive(Args)]
struct TxSub {
    #[command(subcommand)]
    action: TxCommands,
}

#[derive(Subcommand)]
enum TxCommands {
    FormatXlm { stroops: u64 },
    EstimateFee { base_fee: u32, op_count: u32 },
    ValidateHash { hash: String },
    NormalizeHash { hash: String },
}

#[derive(Subcommand)]
enum XlmCommands {
    ToXlm { stroops: u64 },
    ToStroops { xlm: f64 },
    Format { stroops: u64 },
}

fn json_ok(data: serde_json::Value) {
    let out = serde_json::json!({"success": true, "data": data});
    println!("{}", out);
}

fn json_err(msg: &str) {
    let out = serde_json::json!({"success": false, "error": msg});
    println!("{}", out);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ValidateAddress { address } => {
            match validate_address(&address) {
                Ok(_) => {
                    println!("Address is valid: {}", address);
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        
        Commands::Address(sub) => match sub.action {
            AddressCommands::Validate { address } => match validate_address(&address) {
                Ok(_) => {
                    if cli.json {
                        json_ok(serde_json::json!({"valid": true, "address": address}));
                        process::exit(0);
                    }
                    println!("Address is valid: {}", address);
                    process::exit(0);
                }
                Err(e) => {
                    if cli.json {
                        json_err(&e.to_string());
                        process::exit(1);
                    }
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            },
            AddressCommands::Mask { address } => match validate_address(&address) {
                Ok(_) => {
                    if cli.json {
                        json_ok(serde_json::json!(mask_address(&address)));
                        process::exit(0);
                    }
                    println!("{}", mask_address(&address));
                    process::exit(0);
                }
                Err(e) => {
                    if cli.json {
                        json_err(&e.to_string());
                        process::exit(1);
                    }
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            },
            AddressCommands::DetectType { address } => {
                match detect_address_type(&address) {
                    AddressType::Account => {
                        if cli.json {
                            json_ok(serde_json::json!("Account"));
                            process::exit(0);
                        }
                        println!("Account");
                        process::exit(0);
                    }
                    AddressType::Contract => {
                        if cli.json {
                            json_ok(serde_json::json!("Contract"));
                            process::exit(0);
                        }
                        println!("Contract");
                        process::exit(0);
                    }
                    AddressType::Invalid => {
                        if cli.json {
                            json_ok(serde_json::json!("Invalid"));
                            process::exit(0);
                        }
                        eprintln!("Error: Address is invalid");
                        process::exit(1);
                    }
                }
            }
        },
        Commands::Hash(sub) => {
            if let Some(action) = sub.action {
                match action {
                    HashCommands::Sha256 { input } => {
                        let digest = sha256_hex(input.as_bytes());
                        if cli.json { json_ok(serde_json::json!(digest)); process::exit(0); }
                        println!("{}", digest);
                        process::exit(0);
                    }
                    HashCommands::Sha512 { input } => {
                        let digest = sha512_hex(input.as_bytes());
                        if cli.json { json_ok(serde_json::json!(digest)); process::exit(0); }
                        println!("{}", digest);
                        process::exit(0);
                    }
                    HashCommands::DoubleSha256 { input } => {
                        let digest = double_sha256(input.as_bytes());
                        if cli.json { json_ok(serde_json::json!(digest)); process::exit(0); }
                        println!("{}", digest);
                        process::exit(0);
                    }
                }
            }

            // Legacy form: hash <input> --algo <algo>
            if let (Some(input), Some(algo)) = (sub.input, sub.algo) {
                let digest = match algo.as_str() {
                    "sha256" => sha256_hex(input.as_bytes()),
                    "sha512" => sha512_hex(input.as_bytes()),
                    "double-sha256" => double_sha256(input.as_bytes()),
                    _ => { eprintln!("Unknown algorithm"); process::exit(2); }
                };
                println!("{}", digest);
                process::exit(0);
            }

            eprintln!("Invalid hash command");
            process::exit(2);
        }
        Commands::Encode(sub) => {
            if let Some(action) = sub.action {
                match action {
                    EncodeCommands::ToHex { input } => {
                        let out = to_hex(input.as_bytes());
                        if cli.json { json_ok(serde_json::json!(out)); process::exit(0); }
                        println!("{}", out);
                        process::exit(0);
                    }
                    EncodeCommands::FromHex { input } => match from_hex(&input) {
                        Ok(bytes) => {
                            let s = String::from_utf8_lossy(&bytes).to_string();
                            if cli.json { json_ok(serde_json::json!(s)); process::exit(0); }
                            println!("{}", s);
                            process::exit(0);
                        }
                        Err(e) => {
                            if cli.json { json_err(&e.to_string()); process::exit(1); }
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    },
                    EncodeCommands::ToBase64 { input } => {
                        let out = to_base64(input.as_bytes());
                        if cli.json { json_ok(serde_json::json!(out)); process::exit(0); }
                        println!("{}", out);
                        process::exit(0);
                    }
                    EncodeCommands::FromBase64 { input } => match from_base64(&input) {
                        Ok(bytes) => {
                            let s = String::from_utf8_lossy(&bytes).to_string();
                            if cli.json { json_ok(serde_json::json!(s)); process::exit(0); }
                            println!("{}", s);
                            process::exit(0);
                        }
                        Err(e) => {
                            if cli.json { json_err(&e.to_string()); process::exit(1); }
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    },
                }
            }

            // Legacy form: encode <input> --format <format>
            if let (Some(input), Some(format)) = (sub.input, sub.format) {
                match format.as_str() {
                    "hex" => { println!("{}", to_hex(input.as_bytes())); process::exit(0); }
                    "base64" => { println!("{}", to_base64(input.as_bytes())); process::exit(0); }
                    _ => { eprintln!("Unknown format"); process::exit(2); }
                }
            }

            eprintln!("Invalid encode command");
            process::exit(2);
        },
        Commands::Decode { input, format } => {
            match format.as_str() {
                "hex" => match from_hex(&input) {
                    Ok(b) => println!("{}", String::from_utf8_lossy(&b)),
                    Err(e) => { eprintln!("Error: {}", e); process::exit(1); }
                },
                "base64" => match from_base64(&input) {
                    Ok(b) => println!("{}", String::from_utf8_lossy(&b)),
                    Err(e) => { eprintln!("Error: {}", e); process::exit(1); }
                },
                _ => { eprintln!("Unknown format"); process::exit(2); }
            }
            process::exit(0);
        }
        Commands::Tx(sub) => match sub.action {
            TxCommands::FormatXlm { stroops } => {
                let out = format_xlm(stroops);
                if cli.json {
                    json_ok(serde_json::json!(out));
                    process::exit(0);
                }
                println!("{}", out);
                process::exit(0);
            }
            TxCommands::EstimateFee { base_fee, op_count } => {
                let stroops = estimate_fee(base_fee, op_count);
                let xlm = estimate_fee_xlm(base_fee, op_count);
                if cli.json {
                    json_ok(serde_json::json!({"stroops": stroops, "xlm": format!("{:.7} XLM", xlm)}));
                    process::exit(0);
                }
                println!("{} stroops ({} XLM)", stroops, format_xlm(stroops as u64));
                process::exit(0);
            }
            TxCommands::ValidateHash { hash } => {
                let valid = is_valid_tx_hash(&hash);
                if cli.json {
                    if valid {
                        json_ok(serde_json::json!({"valid": valid}));
                        process::exit(0);
                    } else {
                        json_err("Invalid transaction hash");
                        process::exit(1);
                    }
                }
                println!("{}", if valid { "valid" } else { "invalid" });
                process::exit(if valid {0} else {1});
            }
            TxCommands::NormalizeHash { hash } => match normalize_tx_hash(&hash) {
                Ok(h) => {
                    if cli.json {
                        json_ok(serde_json::json!(h));
                        process::exit(0);
                    }
                    println!("{}", h);
                    process::exit(0);
                }
                Err(e) => {
                    if cli.json {
                        json_err(&e.to_string());
                        process::exit(1);
                    }
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            },
        },
        
        Commands::Xlm { action } => match action {
            XlmCommands::ToXlm { stroops } => { println!("{}", soroban_toolkit::transaction::stroops_to_xlm(stroops)); process::exit(0); }
            XlmCommands::ToStroops { xlm } => { println!("{}", soroban_toolkit::transaction::xlm_to_stroops(xlm)); process::exit(0); }
            XlmCommands::Format { stroops } => { println!("{}", format_xlm(stroops)); process::exit(0); }
        },
    }
}
