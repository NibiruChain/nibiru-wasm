use cosmwasm_std::{Uint64, Uint128, Event, Decimal};

pub const POSITION_OPENED_EVENT_NAME: &str = "position_opened";
pub const POSITION_CLOSED_EVENT_NAME: &str = "position_closed";

pub fn new_position_opened_event(pair: String, side: Uint64, quote_asset_amount: Uint128, leverage: Decimal, base_asset_amount_limit: Uint128) -> Event {
    Event::new(POSITION_OPENED_EVENT_NAME)
        .add_attribute("id", pair)
        .add_attribute("side", side.to_string())
        .add_attribute("quote_asset_amount", quote_asset_amount.to_string())
        .add_attribute("leverage", leverage.to_string())
        .add_attribute("base_asset_amount_limit", base_asset_amount_limit.to_string())
}
