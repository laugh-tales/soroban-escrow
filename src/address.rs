use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct SorobanAddress(String);

impl SorobanAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SorobanAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub enum AddressError {
    InvalidLength,
    InvalidPrefix,
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressError::InvalidLength => write!(f, "Address has invalid length"),
            AddressError::InvalidPrefix => write!(f, "Address must start with G or C"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AddressType {
    Account,
    Contract,
    Invalid,
}

/// Validates a Stellar/Soroban address
pub fn validate_address(address: &str) -> Result<SorobanAddress, AddressError> {
    let first = address.chars().next().unwrap_or_default();
    if first != 'G' && first != 'C' {
        return Err(AddressError::InvalidPrefix);
    }
    if address.len() != 56 {
        return Err(AddressError::InvalidLength);
    }
    Ok(SorobanAddress(address.to_string()))
}

/// Returns true if address is a contract address
pub fn is_contract_address(address: &str) -> bool {
    address.starts_with('C')
}

/// Returns true if address is an account address
pub fn is_account_address(address: &str) -> bool {
    address.starts_with('G')
}

/// Masks an address showing only first 4 and last 4 characters
pub fn mask_address(address: &str) -> String {
    if address.len() < 8 {
        return address.to_string();
    }
    format!("{}...{}", &address[..4], &address[address.len() - 4..])
}

/// Detects the type of a Stellar address
pub fn detect_address_type(address: &str) -> AddressType {
    match validate_address(address) {
        Ok(_) if is_contract_address(address) => AddressType::Contract,
        Ok(_) => AddressType::Account,
        Err(_) => AddressType::Invalid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exactly 56 characters, starts with G
    const VALID_ACCOUNT: &str = "GCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";

    // Exactly 56 characters, starts with C
    const VALID_CONTRACT: &str = "CCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UMG";

    #[test]
    fn test_valid_account_address() {
        assert!(validate_address(VALID_ACCOUNT).is_ok());
    }

    #[test]
    fn test_invalid_length() {
        assert_eq!(validate_address("GSHORT"), Err(AddressError::InvalidLength));
    }

    #[test]
    fn test_invalid_prefix() {
        assert_eq!(
            validate_address("XCEZWKCA5VLDNRLN3RPRJMRZOX3Z6G5CHCGZN36UWBE5XFGT35JA5UM"),
            Err(AddressError::InvalidPrefix)
        );
    }

    #[test]
    fn test_mask_address() {
        assert_eq!(mask_address(VALID_ACCOUNT), "GCEZ...5UMG");
    }

    #[test]
    fn test_detect_address_type_account() {
        assert_eq!(detect_address_type(VALID_ACCOUNT), AddressType::Account);
    }

    #[test]
    fn test_detect_address_type_contract() {
        assert_eq!(detect_address_type(VALID_CONTRACT), AddressType::Contract);
    }

    #[test]
    fn test_detect_address_type_invalid() {
        assert_eq!(detect_address_type("invalid"), AddressType::Invalid);
    }

    // Property‑based tests omitted because the `proptest` crate cannot be compiled without the MSVC linker.
    // The tests are retained in the repository history for future use.
}
