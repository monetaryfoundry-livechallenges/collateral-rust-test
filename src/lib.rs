#[derive(Debug, PartialEq)]
pub enum RiskError {
    InvalidLtv,
    Overflow,
}

#[derive(Debug, PartialEq)]
pub struct RiskResult {
    pub allowed: bool,
    pub total_debt: u128,
    pub max_debt: u128,
}

mod market;
mod position;

pub use market::{find_market, markets, Market};
pub use position::{find_position, positions, Position};

/// Check whether a proposed borrow remains within the market's LTV limit.
#[allow(unused_variables)]
pub fn evaluate_borrow(
    collateral: u128,
    existing_debt: u128,
    additional: u128,
    ltv_bps: u128,
) -> Result<RiskResult, RiskError> {
    // TODO: validate ltv_bps <= 10_000, use checked arithmetic, and calculate the result.
    Err(RiskError::InvalidLtv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_inside_limit() {
        let result = evaluate_borrow(1_000, 500, 200, 7_500).unwrap();
        assert!(result.allowed);
        assert_eq!(result.max_debt, 750);
        assert_eq!(result.total_debt, 700);
    }

    #[test]
    fn borrow_at_limit() {
        let result = evaluate_borrow(1_000, 500, 250, 7_500).unwrap();
        assert!(result.allowed);
        assert_eq!(result.total_debt, 750);
    }

    #[test]
    fn borrow_over_limit() {
        let result = evaluate_borrow(1_000, 500, 300, 7_500).unwrap();
        assert!(!result.allowed);
        assert_eq!(result.total_debt, 800);
    }

    #[test]
    fn rejects_invalid_ltv() {
        assert_eq!(
            evaluate_borrow(100, 0, 1, 10_001),
            Err(RiskError::InvalidLtv)
        );
    }

    #[test]
    fn rejects_debt_overflow() {
        assert_eq!(
            evaluate_borrow(u128::MAX, u128::MAX, 1, 10_000),
            Err(RiskError::Overflow)
        );
    }
}
