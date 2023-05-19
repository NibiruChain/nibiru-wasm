use std::str::FromStr;

use cosmwasm_std::{Decimal, Uint64};

pub fn dec_420() -> Decimal {
    Decimal::from_str("420").unwrap()
}
pub fn dec_69() -> Decimal {
    Decimal::from_str("69").unwrap()
}
pub fn u64_420() -> Uint64 {
    Uint64::new(420u64)
}

pub static DUMMY_ADDR: &str = "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl";
pub static DUMMY_ADDR_2: &str = "nibi1ah8gqrtjllhc5ld4rxgl4uglvwl93ag0sh6e6v";
pub static DUMMY_PAIR: &str = "ETH:USD";
