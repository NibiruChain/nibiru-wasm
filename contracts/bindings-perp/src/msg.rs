use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
/// NibiruExecuteMsgWrapper is an override of CosmosMsg::Custom. Using this msg
/// wrapper for the ExecuteMsg handlers show that their return values are valid
/// instances of CosmosMsg::Custom in a type-safe manner. It also shows how
/// CosmosMsg::Custom can be extended in the contract.
/// ExecuteMsg can be extended in the contract.
pub struct NibiruExecuteMsgWrapper {
    pub route: NibiruRoute,
    pub msg: ExecuteMsg,
}

impl CustomMsg for NibiruExecuteMsgWrapper {}

/// "From" is the workforce function for returning messages as fields of the
/// CosmosMsg enum type more easily.
impl From<NibiruExecuteMsgWrapper> for CosmosMsg<NibiruExecuteMsgWrapper> {
    fn from(original: NibiruExecuteMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[cw_serde]
/// Routes here refer to groups of modules on Nibiru. The idea here is to add
/// information on which module or group of modules a particular execute message  
/// belongs to.
pub enum NibiruRoute {
    Perp,
}

#[cw_serde]
pub enum ExecuteMsg {
    OpenPosition {
        sender: String,
        pair: String,
        is_long: bool,
        quote_amount: Uint128,
        leverage: Decimal,
        base_amount_limit: Uint128,
    },

    ClosePosition {
        sender: String,
        pair: String,
    },

    AddMargin {
        sender: String,
        pair: String,
        margin: Coin,
    },

    RemoveMargin {
        sender: String,
        pair: String,
        margin: Coin,
    },

    MultiLiquidate {
        pair: String,
        liquidations: Vec<LiquidationArgs>,
    },

    DonateToInsuranceFund {
        sender: String,
        donation: Coin,
    },

    PegShift {
        pair: String,
        peg_mult: Decimal,
    },
}

#[cw_serde]
pub struct LiquidationArgs {
    pub pair: String,
    pub trader: String,
}

pub fn msg_open_position(
    sender: String,
    pair: String,
    is_long: bool,
    quote_amount: Uint128,
    leverage: Decimal,
    base_amount_limit: Uint128,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::OpenPosition {
            sender,
            pair,
            is_long,
            quote_amount,
            leverage,
            base_amount_limit,
        },
    }
    .into()
}

pub fn msg_close_position(
    sender: String,
    pair: String,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::ClosePosition { sender, pair },
    }
    .into()
}

pub fn msg_add_margin(
    sender: String,
    pair: String,
    margin: Coin,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::AddMargin {
            sender,
            pair,
            margin,
        },
    }
    .into()
}

pub fn msg_remove_margin(
    sender: String,
    pair: String,
    margin: Coin,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::RemoveMargin {
            sender,
            pair,
            margin,
        },
    }
    .into()
}

pub fn msg_multi_liquidate(
    pair: String,
    liquidations: Vec<LiquidationArgs>,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::MultiLiquidate { pair, liquidations },
    }
    .into()
}

pub fn msg_donate_to_insurance_fund(
    sender: String,
    donation: Coin,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::DonateToInsuranceFund { sender, donation },
    }
    .into()
}

pub fn msg_peg_shift(
    pair: String,
    peg_mult: Decimal,
) -> CosmosMsg<NibiruExecuteMsgWrapper> {
    NibiruExecuteMsgWrapper {
        route: NibiruRoute::Perp,
        msg: ExecuteMsg::PegShift { pair, peg_mult },
    }
    .into()
}

#[cfg(test)]
pub mod dummy {

    use std::{fs::File, io::Write, str::FromStr};

    use crate::query::dummy::dec_420;

    use super::*;

    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{Coin, Decimal, Uint128};

    #[cw_serde]
    pub struct ExampleExecuteMsgJson {
        open_position: ExecuteMsg,
        close_position: ExecuteMsg,
        add_margin: ExecuteMsg,
        remove_margin: ExecuteMsg,
        multi_liquidate: ExecuteMsg,
        donate_to_insurance_fund: ExecuteMsg,
        peg_shift: ExecuteMsg,
        depth_shift: ExecuteMsg,
    }

    #[test]
    fn save_example_json() {
        let example_msgs = ExampleExecuteMsgJson {
            open_position: execute_open_position(),
            close_position: execute_close_position(),
            add_margin: execute_add_margin(),
            remove_margin: execute_remove_margin(),
            multi_liquidate: execute_multi_liquidate(),
            donate_to_insurance_fund: execute_donate_to_insurance_fund(),
            peg_shift: execute_peg_shift(),
            depth_shift: execute_depth_shift(),
        };
        let json_str = serde_json::to_string_pretty(&example_msgs).unwrap();
        let mut file = File::create("./execute_msg.json").unwrap();
        assert!(file.write_all(json_str.as_bytes()).is_ok());
    }

    pub static DUMMY_ADDR: &str = "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl";
    pub static DUMMY_ADDR_2: &str =
        "nibi1ah8gqrtjllhc5ld4rxgl4uglvwl93ag0sh6e6v";
    pub static DUMMY_PAIR: &str = "ETH:USD";

    pub fn execute_open_position() -> ExecuteMsg {
        ExecuteMsg::OpenPosition {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
            is_long: true,
            quote_amount: Uint128::from(100u128),
            leverage: Decimal::from_str("5").unwrap(),
            base_amount_limit: Uint128::from(1000u128),
        }
    }

    pub fn execute_close_position() -> ExecuteMsg {
        ExecuteMsg::ClosePosition {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
        }
    }

    pub fn execute_add_margin() -> ExecuteMsg {
        ExecuteMsg::AddMargin {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
            margin: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
        }
    }

    pub fn execute_remove_margin() -> ExecuteMsg {
        ExecuteMsg::RemoveMargin {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
            margin: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
        }
    }

    pub fn execute_multi_liquidate() -> ExecuteMsg {
        ExecuteMsg::MultiLiquidate {
            pair: DUMMY_PAIR.to_string(),
            liquidations: vec![
                LiquidationArgs {
                    pair: DUMMY_PAIR.to_string(),
                    trader: DUMMY_ADDR.to_string(),
                },
                LiquidationArgs {
                    pair: DUMMY_PAIR.to_string(),
                    trader: DUMMY_ADDR_2.to_string(),
                },
            ],
        }
    }

    pub fn execute_donate_to_insurance_fund() -> ExecuteMsg {
        ExecuteMsg::DonateToInsuranceFund {
            sender: DUMMY_ADDR.to_string(),
            donation: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
        }
    }

    pub fn execute_peg_shift() -> ExecuteMsg {
        ExecuteMsg::PegShift {
            pair: DUMMY_PAIR.to_string(),
            peg_mult: dec_420(),
        }
    }

    pub fn execute_depth_shift() -> ExecuteMsg {
        ExecuteMsg::DepthShift {
            pair: DUMMY_PAIR.to_string(),
            depth_mult: dec_420(),
        }
    }

    pub fn liquidation_args() -> LiquidationArgs {
        LiquidationArgs {
            pair: DUMMY_PAIR.to_string(),
            trader: "trader".to_string(),
        }
    }
}
