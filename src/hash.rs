use sha2::{Digest, Sha256, Sha512};

/// Returns SHA-256 hash as hex string
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Returns SHA-256 hash as bytes
pub fn sha256_bytes(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Returns SHA-512 hash as hex string
pub fn sha512_hex(data: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Returns BLAKE3 hash as hex string
///
/// # Examples
///
/// ```
/// use soroban_toolkit::hash::blake3_hex;
///
/// let result = blake3_hex(b"hello");
/// assert_eq!(result, "ea8f163db38682925e4491c5e58d41a79a83e864690e4dd163deb6a9b4480e48");
/// ```
pub fn blake3_hex(data: &[u8]) -> String {
    hex::encode(blake3::hash(data).as_bytes())
}

/// Returns BLAKE3 hash as bytes
///
/// # Examples
///
/// ```
/// use soroban_toolkit::hash::blake3_bytes;
/// use hex;
///
/// let result = blake3_bytes(b"hello");
/// let expected_hex = "ea8f163db38682925e4491c5e58d41a79a83e864690e4dd163deb6a9b4480e48";
/// assert_eq!(hex::encode(result), expected_hex);
/// ```
pub fn blake3_bytes(data: &[u8]) -> Vec<u8> {
    blake3::hash(data).as_bytes().to_vec()
}

/// Returns double SHA-256 hash as hex string (used in blockchain contexts like Bitcoin)
///
/// # Examples
///
/// ```
/// use soroban_toolkit::hash::double_sha256;
///
/// let result = double_sha256(b"hello");
/// assert_eq!(result, "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
/// ```
pub fn double_sha256(data: &[u8]) -> String {
    let first = sha256_bytes(data);
    sha256_hex(&first)
}

/// Timing-safe comparison of two byte slices
pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_known_value() {
        let result = sha256_hex(b"hello");
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha512_not_empty() {
        let result = sha512_hex(b"hello");
        assert!(!result.is_empty());
        assert_eq!(result.len(), 128);
    }

    #[test]
    fn test_double_sha256() {
        let result = double_sha256(b"hello");
        assert_eq!(result, "9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_secure_compare_equal() {
        assert!(secure_compare(b"hello", b"hello"));
    }

    #[test]
    fn test_secure_compare_not_equal() {
        assert!(!secure_compare(b"hello", b"world"));
    }

    #[test]
    fn test_blake3_hex_known_value() {
        let result = blake3_hex(b"hello");
        assert_eq!(
            result,
            "ea8f163db38682925e4491c5e58d41a79a83e864690e4dd163deb6a9b4480e48"
        );
    }

    #[test]
    fn test_blake3_bytes_known_value() {
        let result = blake3_bytes(b"hello");
        let expected_hex = "ea8f163db38682925e4491c5e58d41a79a83e864690e4dd163deb6a9b4480e48";
        assert_eq!(hex::encode(result), expected_hex);
    }
}
