#[derive(Debug, PartialEq)]
pub struct Market {
    pub id: &'static str,
    pub collateral_symbol: &'static str,
    pub debt_symbol: &'static str,
    pub max_ltv_bps: u128,
}

pub fn markets() -> &'static [Market] {
    &[
        Market {
            id: "eth-usdc",
            collateral_symbol: "ETH",
            debt_symbol: "USDC",
            max_ltv_bps: 7_500,
        },
        Market {
            id: "wbtc-usdc",
            collateral_symbol: "WBTC",
            debt_symbol: "USDC",
            max_ltv_bps: 7_000,
        },
        Market {
            id: "steth-usdc",
            collateral_symbol: "stETH",
            debt_symbol: "USDC",
            max_ltv_bps: 6_500,
        },
    ]
}

pub fn find_market(id: &str) -> Option<&'static Market> {
    markets().iter().find(|market| market.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_the_eth_market() {
        let market = find_market("eth-usdc").expect("eth market");
        assert_eq!(market.max_ltv_bps, 7_500);
        assert_eq!(markets().len(), 3);
        assert!(find_market("missing").is_none());
    }
}
