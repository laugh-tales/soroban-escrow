use base64::{engine::general_purpose, Engine as _};

#[derive(Debug)]
pub enum EncodingError {
    InvalidHex,
    InvalidBase64,
    InvalidJson,
}

impl std::fmt::Display for EncodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::InvalidHex => write!(f, "Invalid hex string"),
            EncodingError::InvalidBase64 => write!(f, "Invalid base64 string"),
            EncodingError::InvalidJson => write!(f, "Invalid JSON string"),
        }
    }
}

/// Encodes bytes as hex string
pub fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Decodes hex string to bytes
pub fn from_hex(s: &str) -> Result<Vec<u8>, EncodingError> {
    hex::decode(s).map_err(|_| EncodingError::InvalidHex)
}

/// Encodes bytes as standard base64
pub fn to_base64(bytes: &[u8]) -> String {
    general_purpose::STANDARD.encode(bytes)
}

/// Decodes standard base64 to bytes
pub fn from_base64(s: &str) -> Result<Vec<u8>, EncodingError> {
    general_purpose::STANDARD
        .decode(s)
        .map_err(|_| EncodingError::InvalidBase64)
}

/// Encodes bytes as URL-safe base64
pub fn to_base64_url(bytes: &[u8]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// Decodes URL-safe base64 to bytes
pub fn from_base64_url(s: &str) -> Result<Vec<u8>, EncodingError> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| EncodingError::InvalidBase64)
}

/// Pretty prints a JSON string
pub fn pretty_print_json(json: &str) -> Result<String, EncodingError> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|_| EncodingError::InvalidJson)?;
    serde_json::to_string_pretty(&value).map_err(|_| EncodingError::InvalidJson)
}

/// Splits a byte slice into fixed-size chunks.
///
/// Returns a `Vec` of owned `Vec<u8>` chunks in order. If `chunk_size` is
/// zero this function returns an empty `Vec` to avoid a panic. If `chunk_size`
/// is larger than the input, a single chunk containing the whole input is
/// returned. This is useful for streaming or processing large contract data in
/// fixed-size pieces.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let data = b"helloworld";
/// let chunks = soroban_toolkit::encoding::chunk_bytes(data, 5);
/// assert_eq!(chunks, vec![b"hello".to_vec(), b"world".to_vec()]);
/// ```
///
/// Empty input yields no chunks:
///
/// ```rust
/// let chunks = soroban_toolkit::encoding::chunk_bytes(&[], 4);
/// assert!(chunks.is_empty());
/// ```
pub fn chunk_bytes(data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
    if chunk_size == 0 {
        return Vec::new();
    }
    data.chunks(chunk_size).map(|c| c.to_vec()).collect()
}

/// Reassembles a sequence of chunks produced by `chunk_bytes` back into a
/// single contiguous `Vec<u8>`.
///
/// Panics are not possible here; an empty slice of chunks returns an empty
/// vector.
///
/// # Examples
///
/// ```rust
/// let data = b"helloworld";
/// let chunks = soroban_toolkit::encoding::chunk_bytes(data, 5);
/// let reassembled = soroban_toolkit::encoding::reassemble_chunks(&chunks);
/// assert_eq!(reassembled, data);
/// ```
pub fn reassemble_chunks(chunks: &[Vec<u8>]) -> Vec<u8> {
    chunks.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_roundtrip() {
        let original = b"soroban";
        let encoded = to_hex(original);
        let decoded = from_hex(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_base64_roundtrip() {
        let original = b"laugh-tales";
        let encoded = to_base64(original);
        let decoded = from_base64(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_base64_url_roundtrip() {
        let original = b"laugh-tales";
        let encoded = to_base64_url(original);
        let decoded = from_base64_url(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_chunk_and_reassemble() {
        let data = b"helloworldsoroban";
        let chunks = chunk_bytes(data, 5);
        let reassembled = reassemble_chunks(&chunks);
        assert_eq!(reassembled, data);
    }

    #[test]
    fn test_chunk_empty_input() {
        let data: &[u8] = &[];
        let chunks = chunk_bytes(data, 4);
        assert!(chunks.is_empty());
        let reassembled = reassemble_chunks(&chunks);
        assert!(reassembled.is_empty());
    }

    #[test]
    fn test_chunk_larger_than_data() {
        let data = b"abc";
        let chunks = chunk_bytes(data, 10);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], data);
        let reassembled = reassemble_chunks(&chunks);
        assert_eq!(reassembled, data);
    }

    #[test]
    fn test_chunk_zero_size_returns_empty() {
        let data = b"abcdef";
        let chunks = chunk_bytes(data, 0);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_pretty_print_json() {
        let json = r#"{"name":"soroban","version":"0.1.0"}"#;
        let pretty = pretty_print_json(json).unwrap();
        assert!(pretty.contains('\n'));
    }
}
