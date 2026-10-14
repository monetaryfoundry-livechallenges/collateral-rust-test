#[derive(Debug, PartialEq)]
pub struct Position {
    pub id: &'static str,
    pub market_id: &'static str,
    pub owner: &'static str,
    pub collateral_value: u128,
    pub debt: u128,
}

pub fn positions() -> &'static [Position] {
    &[
        Position {
            id: "pos-1001",
            market_id: "eth-usdc",
            owner: "alice",
            collateral_value: 10_000,
            debt: 5_200,
        },
        Position {
            id: "pos-1002",
            market_id: "wbtc-usdc",
            owner: "bob",
            collateral_value: 1_000,
            debt: 500,
        },
        Position {
            id: "pos-1003",
            market_id: "steth-usdc",
            owner: "carol",
            collateral_value: 2_500,
            debt: 0,
        },
    ]
}

pub fn find_position(id: &str) -> Option<&'static Position> {
    positions().iter().find(|position| position.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_bob() {
        let position = find_position("pos-1002").expect("position");
        assert_eq!(position.owner, "bob");
        assert_eq!(position.debt, 500);
        assert!(find_position("missing").is_none());
    }
}
