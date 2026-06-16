use soroban_toolkit::{
    address::{detect_address_type, mask_address, validate_address, AddressType},
    encoding::{from_base64, from_hex, to_base64, to_hex},
    hash::{secure_compare, sha256_bytes, sha256_hex},
    transaction::{estimate_fee, format_xlm, normalize_tx_hash},
};

const VALID_ACCOUNT: &str = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";
const VALID_CONTRACT: &str = "CCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";

#[test]
fn address_validation_output_can_be_hashed_deterministically() {
    let address = validate_address(VALID_ACCOUNT).expect("valid account address");

    let digest = sha256_hex(address.as_str().as_bytes());

    assert_eq!(detect_address_type(address.as_str()), AddressType::Account);
    assert_eq!(
        digest,
        "87bd3cc5a54936558792321c2be120c1919ca34e745baeefaa67d18a7bf854f2"
    );
}

#[test]
fn encoded_hash_bytes_roundtrip_without_changing_digest() {
    let digest = sha256_bytes(VALID_ACCOUNT.as_bytes());

    let hex_digest = to_hex(&digest);
    let decoded_digest = from_hex(&hex_digest).expect("valid hex digest");

    assert_eq!(hex_digest.len(), 64);
    assert!(secure_compare(&digest, &decoded_digest));
}

#[test]
fn address_bytes_survive_base64_encoding_roundtrip() {
    let address = validate_address(VALID_CONTRACT).expect("valid contract address");

    let encoded = to_base64(address.as_str().as_bytes());
    let decoded = from_base64(&encoded).expect("valid base64 address");
    let decoded_address = String::from_utf8(decoded).expect("address bytes are valid UTF-8");

    assert_eq!(decoded_address, address.as_str());
    assert_eq!(detect_address_type(&decoded_address), AddressType::Contract);
    assert_eq!(mask_address(&decoded_address), "CCEZ...5UMG");
}

#[test]
fn validated_address_can_drive_encoded_transaction_metadata_flow() {
    let address = validate_address(VALID_ACCOUNT).expect("valid account address");
    let digest = sha256_hex(address.as_str().as_bytes());

    let normalized_hash =
        normalize_tx_hash(&format!("0x{digest}")).expect("valid transaction hash");
    let encoded_hash = to_base64(normalized_hash.as_bytes());
    let decoded_hash = from_base64(&encoded_hash).expect("valid base64 transaction hash");

    assert_eq!(normalized_hash, digest);
    assert_eq!(decoded_hash, normalized_hash.as_bytes());
    assert_eq!(estimate_fee(100, 3), 300);
    assert_eq!(format_xlm(300), "0.0000300 XLM");
}
