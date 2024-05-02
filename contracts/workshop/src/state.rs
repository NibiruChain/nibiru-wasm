use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct State {
    pub count: i32,
}

pub const COUNT: Item<State> = Item::new("state");
