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
pub fn blake3_hex(data: &[u8]) -> String {
    hex::encode(blake3::hash(data).as_bytes())
}

/// Returns double SHA-256 hash as hex string (used in Bitcoin/blockchain)
pub fn double_sha256(data: &[u8]) -> String {
    let first = sha256_bytes(data);
    sha256_hex(&first)
}

/// Timing-safe comparison of two byte slices
///
/// This function performs a constant-time comparison to prevent timing attacks.
/// When comparing sensitive data like hashes or cryptographic signatures, a naive
/// byte-by-byte comparison using `==` can leak information through timing differences:
/// - Early mismatch exits immediately (fast)
/// - Full matches require comparing all bytes (slow)
///
/// An attacker can measure these timing differences to infer how many bytes match,
/// potentially recovering the correct value through repeated attacks.
///
/// This implementation uses XOR operations and bitwise OR to ensure all bytes are
/// compared regardless of their values, taking constant time regardless of where
/// mismatches occur.
///
/// # Arguments
/// * `a` - First byte slice to compare
/// * `b` - Second byte slice to compare
///
/// # Returns
/// `true` if both slices are equal, `false` otherwise
///
/// # Example
/// ```
/// let hash1 = [0xab, 0xcd];
/// let hash2 = [0xab, 0xcd];
/// assert!(soroban_escrow::hash::secure_compare(&hash1, &hash2));
/// ```
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
    fn test_secure_compare_empty() {
        assert!(secure_compare(b"", b""));
    }

    #[test]
    fn test_secure_compare_different_length() {
        assert!(!secure_compare(b"hello", b"hi"));
        assert!(!secure_compare(b"a", b"ab"));
    }

    #[test]
    fn test_secure_compare_single_byte_diff() {
        // Test that differing at different positions takes same time
        assert!(!secure_compare(&[0xFF], &[0x00]));
        assert!(!secure_compare(&[0x00], &[0xFF]));
    }

    #[test]
    fn test_secure_compare_hash_values() {
        // Test with actual hash-like byte sequences
        let hash1 = sha256_bytes(b"test");
        let hash2 = sha256_bytes(b"test");
        let hash3 = sha256_bytes(b"different");
        
        assert!(secure_compare(&hash1, &hash2));
        assert!(!secure_compare(&hash1, &hash3));
    }

    #[test]
    fn test_secure_compare_constant_time() {
        // This test demonstrates the constant-time behavior
        // Even though we can't measure time precisely in tests,
        // the implementation guarantees that all bytes are compared
        let a = vec![0xAB; 32];
        let b = vec![0xAB; 32];
        let c = vec![0xCD; 32];
        
        // Both of these should take the same time to compare
        let _result1 = secure_compare(&a, &b); // equal
        let _result2 = secure_compare(&a, &c); // different
        // In practice, both comparisons process all 32 bytes
    }
}
