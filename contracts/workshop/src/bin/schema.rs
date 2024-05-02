use cosmwasm_schema::write_api;

use workshop::msgs::QueryMsg;
use workshop::msgs::{ExecuteMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
