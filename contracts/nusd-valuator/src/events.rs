use std::collections::BTreeSet;

use cosmwasm_std::Event;

pub fn event_add_denom(denom: &str, denom_set_json: &str) -> Event {
    Event::new("nusd_valuator/add_denom")
        .add_attribute("denom", denom)
        .add_attribute("new_denom_set", denom_set_json)
}

pub fn event_remove_denom(denom: &str, denom_set_json: &str) -> Event {
    Event::new("nusd_valuator/remove_denom")
        .add_attribute("denom", denom)
        .add_attribute("new_denom_set", denom_set_json)
}

pub fn event_change_denom(
    from_denom: &str,
    to_denom: &str,
    denom_set_json: &str,
) -> Event {
    Event::new("nusd_valuator/change_denom")
        .add_attribute("from_denom", from_denom)
        .add_attribute("to_denom", to_denom)
        .add_attribute("new_denom_set", denom_set_json)
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
