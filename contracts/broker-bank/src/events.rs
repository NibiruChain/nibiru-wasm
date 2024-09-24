use std::collections::BTreeSet;

use cosmwasm_std::Event;

pub fn event_bank_send(coins_json: &str, caller: &str) -> Event {
    Event::new("broker_bank/send")
        .add_attribute("coins", coins_json)
        .add_attribute("caller", caller)
}

pub fn event_toggle_halt(is_halted: &bool) -> Event {
    Event::new("broker_bank/toggle_halt")
        .add_attribute("new_is_halted", is_halted.to_string())
}

pub fn event_withdraw(coins_json: &str, to_addr: &str) -> Event {
    Event::new("broker_bank/withdraw")
        .add_attribute("coins", coins_json)
        .add_attribute("to_addr", to_addr)
}

pub fn denom_set_json(
    denom_set: BTreeSet<String>,
) -> serde_json::Result<String> {
    serde_json::to_string(&denom_set)
}

pub fn event_migrate(_arg0: &u64, _arg1: &bool) -> Event {
    // Event::new("migrate_nusd_valuator")
    //     .add_attribute("id", _arg0.to_string())
    //     .add_attribute("new_done", _arg1.to_string())
    todo!(); // TODO: event migrate
}
