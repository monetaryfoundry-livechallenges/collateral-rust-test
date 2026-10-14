use heldnav_risk_engine::{evaluate_borrow, find_market, markets, positions, RiskError};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("markets") => print_markets(),
        Some("positions") => print_positions(),
        Some("check") => check(&mut args),
        _ => usage(),
    }
}

fn print_markets() -> ExitCode {
    for market in markets() {
        println!(
            "{}\t{}/{}\tmax_ltv_bps={}",
            market.id, market.collateral_symbol, market.debt_symbol, market.max_ltv_bps
        );
    }
    ExitCode::SUCCESS
}

fn print_positions() -> ExitCode {
    for position in positions() {
        let max_ltv = find_market(position.market_id).map(|market| market.max_ltv_bps);
        println!(
            "{}\tmarket={}\towner={}\tcollateral={}\tdebt={}\tmax_ltv_bps={}",
            position.id,
            position.market_id,
            position.owner,
            position.collateral_value,
            position.debt,
            max_ltv.unwrap_or(0)
        );
    }
    ExitCode::SUCCESS
}

fn check(args: &mut impl Iterator<Item = String>) -> ExitCode {
    let Some(collateral) = parse_u128(args.next()) else {
        return usage();
    };
    let Some(existing_debt) = parse_u128(args.next()) else {
        return usage();
    };
    let Some(additional) = parse_u128(args.next()) else {
        return usage();
    };
    let Some(ltv_bps) = parse_u128(args.next()) else {
        return usage();
    };
    match evaluate_borrow(collateral, existing_debt, additional, ltv_bps) {
        Ok(result) => {
            println!(
                "allowed={}\tmax_debt={}\ttotal_debt={}",
                result.allowed, result.max_debt, result.total_debt
            );
            ExitCode::SUCCESS
        }
        Err(RiskError::InvalidLtv) => {
            eprintln!("invalid ltv");
            ExitCode::from(1)
        }
        Err(RiskError::Overflow) => {
            eprintln!("overflow");
            ExitCode::from(1)
        }
    }
}

fn parse_u128(value: Option<String>) -> Option<u128> {
    value?.parse().ok()
}

fn usage() -> ExitCode {
    eprintln!("usage:");
    eprintln!("  heldnav markets");
    eprintln!("  heldnav positions");
    eprintln!("  heldnav check <collateral> <existing_debt> <additional> <ltv_bps>");
    ExitCode::from(2)
}
