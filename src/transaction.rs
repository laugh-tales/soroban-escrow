#[derive(Debug)]
pub enum TxError {
    InvalidHash,
    InvalidFee,
    InvalidSequence,
    InvalidTimeBounds,
    InvalidAssetCode,
}

impl std::fmt::Display for TxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxError::InvalidHash => write!(f, "Invalid transaction hash"),
            TxError::InvalidFee => write!(f, "Invalid fee"),
            TxError::InvalidSequence => write!(f, "Invalid sequence number"),
            TxError::InvalidTimeBounds => write!(f, "Invalid time bounds: max_time must be 0 or greater than min_time"),
            TxError::InvalidAssetCode => write!(f, "Invalid asset code"),
        }
    }
}

/// Represents the time bounds for a Stellar transaction.
///
/// `min_time` and `max_time` are Unix timestamps in seconds.
/// A value of `0` for `max_time` means the transaction has no expiry.
/// A value of `0` for `min_time` means the transaction is valid from the start of time.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeBounds {
    /// Earliest time (Unix seconds) at which the transaction is valid (0 = no lower bound)
    pub min_time: u64,
    /// Latest time (Unix seconds) at which the transaction is valid (0 = no upper bound)
    pub max_time: u64,
}

/// Validates Stellar transaction time bounds.
///
/// Returns `Ok(())` if the bounds are valid. Returns `Err(TxError::InvalidTimeBounds)`
/// when `max_time` is non-zero and not strictly greater than `min_time`.
///
/// # Examples
///
/// ```rust
/// use soroban_toolkit::transaction::{TimeBounds, validate_time_bounds};
///
/// let valid = TimeBounds { min_time: 1_000, max_time: 2_000 };
/// assert!(validate_time_bounds(&valid).is_ok());
///
/// let no_expiry = TimeBounds { min_time: 1_000, max_time: 0 };
/// assert!(validate_time_bounds(&no_expiry).is_ok());
///
/// let invalid = TimeBounds { min_time: 2_000, max_time: 1_000 };
/// assert!(validate_time_bounds(&invalid).is_err());
/// ```
pub fn validate_time_bounds(bounds: &TimeBounds) -> Result<(), TxError> {
    if bounds.max_time != 0 && bounds.max_time <= bounds.min_time {
        return Err(TxError::InvalidTimeBounds);
    }
    Ok(())
}

/// Returns `true` if `current_time` falls within the given time bounds.
///
/// - Returns `false` when `current_time < min_time`.
/// - Returns `false` when `max_time` is non-zero and `current_time > max_time`.
/// - A `max_time` of `0` means no upper bound (never expires).
///
/// # Examples
///
/// ```rust
/// use soroban_toolkit::transaction::{TimeBounds, is_within_bounds};
///
/// let bounds = TimeBounds { min_time: 1_000, max_time: 2_000 };
/// assert!(is_within_bounds(&bounds, 1_500));
/// assert!(!is_within_bounds(&bounds, 500));
/// assert!(!is_within_bounds(&bounds, 2_500));
///
/// // Boundary values are inclusive
/// assert!(is_within_bounds(&bounds, 1_000));
/// assert!(is_within_bounds(&bounds, 2_000));
///
/// // max_time == 0 means no expiry
/// let open = TimeBounds { min_time: 1_000, max_time: 0 };
/// assert!(is_within_bounds(&open, 99_999_999));
/// ```
pub fn is_within_bounds(bounds: &TimeBounds, current_time: u64) -> bool {
    if current_time < bounds.min_time {
        return false;
    }
    if bounds.max_time != 0 && current_time > bounds.max_time {
        return false;
    }
    true
}

/// Converts stroops to XLM (1 XLM = 10,000,000 stroops)
///
/// # Examples
///
/// ```rust
/// use soroban_toolkit::transaction::stroops_to_xlm;
/// assert_eq!(stroops_to_xlm(10_000_000), 1.0);
/// assert_eq!(stroops_to_xlm(1_500_000), 0.15);
/// assert_eq!(stroops_to_xlm(300), 0.00003);
/// ```
pub fn stroops_to_xlm(stroops: u64) -> f64 {
    stroops as f64 / 10_000_000.0
}

/// Converts XLM to stroops
///
/// # Examples
///
/// ```rust
/// use soroban_toolkit::transaction::xlm_to_stroops;
/// assert_eq!(xlm_to_stroops(1.0), 10_000_000);
/// assert_eq!(xlm_to_stroops(0.15), 1_500_000);
/// assert_eq!(xlm_to_stroops(0.0000001), 1);
/// ```
pub fn xlm_to_stroops(xlm: f64) -> u64 {
    (xlm * 10_000_000.0).round() as u64
}

/// Formats stroops as a readable XLM string
///
/// # Examples
///
/// ```rust
/// use soroban_toolkit::transaction::format_xlm;
/// assert_eq!(format_xlm(10_000_000), "1.0000000 XLM");
/// assert_eq!(format_xlm(1_500_000), "0.1500000 XLM");
/// assert_eq!(format_xlm(300), "0.0000300 XLM");
/// ```
pub fn format_xlm(stroops: u64) -> String {
    format!("{:.7} XLM", stroops_to_xlm(stroops))
}

/// Validates a Stellar transaction hash (64 hex characters)
pub fn is_valid_tx_hash(hash: &str) -> bool {
    hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit())
}

/// Normalizes a transaction hash (lowercase, strip 0x prefix)
pub fn normalize_tx_hash(hash: &str) -> Result<String, TxError> {
    let h = hash.strip_prefix("0x").unwrap_or(hash).to_lowercase();
    if is_valid_tx_hash(&h) {
        Ok(h)
    } else {
        Err(TxError::InvalidHash)
    }
}

/// Estimates transaction fee in stroops
pub fn estimate_fee(base_fee: u32, operation_count: u32) -> u32 {
    base_fee * operation_count
}

/// Estimates transaction fee in XLM
pub fn estimate_fee_xlm(base_fee: u32, operation_count: u32) -> f64 {
    stroops_to_xlm(estimate_fee(base_fee, operation_count) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stroops_to_xlm() {
        assert_eq!(stroops_to_xlm(10_000_000), 1.0);
        assert_eq!(stroops_to_xlm(5_000_000), 0.5);
    }

    #[test]
    fn test_xlm_to_stroops() {
        assert_eq!(xlm_to_stroops(1.0), 10_000_000);
        assert_eq!(xlm_to_stroops(0.5), 5_000_000);
    }

    #[test]
    fn test_format_xlm() {
        assert_eq!(format_xlm(10_000_000), "1.0000000 XLM");
        assert_eq!(format_xlm(1_500_000), "0.1500000 XLM");
        assert_eq!(format_xlm(300), "0.0000300 XLM");
    }

    #[test]
    fn test_stroops_to_xlm_small_values() {
        assert_eq!(stroops_to_xlm(1), 0.0000001);
        assert_eq!(stroops_to_xlm(300), 0.00003);
        assert_eq!(stroops_to_xlm(15_000_000), 1.5);
    }

    #[test]
    fn test_xlm_to_stroops_rounding() {
        assert_eq!(xlm_to_stroops(0.0000001), 1);
        assert_eq!(xlm_to_stroops(1.5), 15_000_000);
        assert_eq!(xlm_to_stroops(0.00003), 300);
    }

    #[test]
    fn test_valid_tx_hash() {
        let hash = "a".repeat(64);
        assert!(is_valid_tx_hash(&hash));
        assert!(!is_valid_tx_hash("short"));
    }

    #[test]
    fn test_estimate_fee() {
        assert_eq!(estimate_fee(100, 3), 300);
    }

    #[test]
    fn test_validate_time_bounds_valid() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 2_000 };
        assert!(validate_time_bounds(&bounds).is_ok());
    }

    #[test]
    fn test_validate_time_bounds_max_less_than_min() {
        let bounds = TimeBounds { min_time: 2_000, max_time: 1_000 };
        assert!(validate_time_bounds(&bounds).is_err());
    }

    #[test]
    fn test_validate_time_bounds_max_equal_to_min() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 1_000 };
        assert!(validate_time_bounds(&bounds).is_err());
    }

    #[test]
    fn test_validate_time_bounds_zero_max_time() {
        // max_time == 0 means no expiry — always valid regardless of min_time
        let bounds = TimeBounds { min_time: 1_000, max_time: 0 };
        assert!(validate_time_bounds(&bounds).is_ok());
    }

    #[test]
    fn test_validate_time_bounds_both_zero() {
        let bounds = TimeBounds { min_time: 0, max_time: 0 };
        assert!(validate_time_bounds(&bounds).is_ok());
    }

    #[test]
    fn test_is_within_bounds_inside_range() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 2_000 };
        assert!(is_within_bounds(&bounds, 1_500));
    }

    #[test]
    fn test_is_within_bounds_before_min() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 2_000 };
        assert!(!is_within_bounds(&bounds, 500));
    }

    #[test]
    fn test_is_within_bounds_after_max() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 2_000 };
        assert!(!is_within_bounds(&bounds, 2_500));
    }

    #[test]
    fn test_is_within_bounds_at_min_boundary() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 2_000 };
        assert!(is_within_bounds(&bounds, 1_000));
    }

    #[test]
    fn test_is_within_bounds_at_max_boundary() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 2_000 };
        assert!(is_within_bounds(&bounds, 2_000));
    }

    #[test]
    fn test_is_within_bounds_no_expiry() {
        let bounds = TimeBounds { min_time: 1_000, max_time: 0 };
        assert!(is_within_bounds(&bounds, 99_999_999));
    }

    #[test]
    fn test_is_within_bounds_zero_min_time() {
        let bounds = TimeBounds { min_time: 0, max_time: 2_000 };
        assert!(is_within_bounds(&bounds, 0));
        assert!(is_within_bounds(&bounds, 1_000));
        assert!(!is_within_bounds(&bounds, 2_001));
    }

    #[test]
    fn test_time_bounds_error_message() {
        let bounds = TimeBounds { min_time: 5_000, max_time: 1_000 };
        let err = validate_time_bounds(&bounds).unwrap_err();
        assert!(err.to_string().contains("max_time"));
    }
}
