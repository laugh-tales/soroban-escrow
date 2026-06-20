/// Calculates recommended transaction fees based on base fee and operation count.
///
/// # Stellar Fee Model
///
/// Stellar transaction fees are calculated as:
/// `fee = base_fee * (number_of_operations + 1)`
///
/// The `+ 1` accounts for the transaction envelope itself.
///
/// # Examples
///
/// ```
/// use soroban_toolkit::fee::estimate_fee;
///
/// // Calculate fee for a single operation at 100 stroops base fee
/// let fee = estimate_fee(100, 1);
/// assert_eq!(fee, 200); // (1 + 1) * 100
///
/// // Calculate fee for 5 operations at 100 stroops base fee
/// let fee = estimate_fee(100, 5);
/// assert_eq!(fee, 600); // (5 + 1) * 100
/// ```
pub fn estimate_fee(base_fee: u32, operation_count: u32) -> u32 {
    base_fee.saturating_mul(operation_count.saturating_add(1))
}

/// Calculates recommended transaction fees in XLM based on base fee and operation count.
///
/// # Stellar Fee Model
///
/// Stellar transaction fees are calculated as:
/// `fee = base_fee * (number_of_operations + 1)`
///
/// The `+ 1` accounts for the transaction envelope itself.
/// One stroops equals 0.0000001 XLM (10^-7 XLM).
///
/// # Examples
///
/// ```
/// use soroban_toolkit::fee::estimate_fee_xlm;
///
/// // Calculate fee for a single operation at 100 stroops base fee
/// let fee = estimate_fee_xlm(100, 1);
/// assert_eq!(fee, 0.00002); // (1 + 1) * 100 stroops = 0.00002 XLM
///
/// // Calculate fee for 5 operations at 100 stroops base fee
/// let fee = estimate_fee_xlm(100, 5);
/// assert_eq!(fee, 0.00006); // (5 + 1) * 100 stroops = 0.00006 XLM
/// ```
pub fn estimate_fee_xlm(base_fee: u32, operation_count: u32) -> f64 {
    let fee_stroops = estimate_fee(base_fee, operation_count);
    fee_stroops as f64 / 10_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_fee_single_operation() {
        let fee = estimate_fee(100, 1);
        assert_eq!(fee, 200);
    }

    #[test]
    fn test_estimate_fee_multiple_operations() {
        let fee = estimate_fee(100, 5);
        assert_eq!(fee, 600);
    }

    #[test]
    fn test_estimate_fee_zero_operations() {
        let fee = estimate_fee(100, 0);
        assert_eq!(fee, 100);
    }

    #[test]
    fn test_estimate_fee_zero_base_fee() {
        let fee = estimate_fee(0, 5);
        assert_eq!(fee, 0);
    }

    #[test]
    fn test_estimate_fee_large_values() {
        let fee = estimate_fee(1000, 1000);
        assert_eq!(fee, 1001000);
    }

    #[test]
    fn test_estimate_fee_xlm_single_operation() {
        let fee = estimate_fee_xlm(100, 1);
        assert_eq!(fee, 0.00002);
    }

    #[test]
    fn test_estimate_fee_xlm_multiple_operations() {
        let fee = estimate_fee_xlm(100, 5);
        assert_eq!(fee, 0.00006);
    }

    #[test]
    fn test_estimate_fee_xlm_zero_operations() {
        let fee = estimate_fee_xlm(100, 0);
        assert_eq!(fee, 0.00001);
    }

    #[test]
    fn test_estimate_fee_xlm_zero_base_fee() {
        let fee = estimate_fee_xlm(0, 5);
        assert_eq!(fee, 0.0);
    }

    #[test]
    fn test_estimate_fee_xlm_large_values() {
        let fee = estimate_fee_xlm(1000, 1000);
        assert_eq!(fee, 0.1001);
    }
}
