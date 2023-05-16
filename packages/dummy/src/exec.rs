#[cfg(test)]
pub mod test {

    use std::{fs::File, io::Write, str::FromStr};

    use crate::common::{dec_420, DUMMY_ADDR, DUMMY_ADDR_2, DUMMY_PAIR};

    use bindings_perp::msg::{ExecuteMsg as NBExecuteMsg, LiquidationArgs};
    use controller::msgs::ExecuteMsg as ControllerExecuteMsg;
    use shifter::msgs::ExecuteMsg as ShifterExecuteMsg;

    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{Coin, Decimal, Uint128, Uint256};

    #[cw_serde]
    pub struct ExampleExecuteMsgJson {
        open_position: NBExecuteMsg,
        close_position: NBExecuteMsg,
        add_margin: NBExecuteMsg,
        remove_margin: NBExecuteMsg,
        multi_liquidate: NBExecuteMsg,
        donate_to_insurance_fund: NBExecuteMsg,

        peg_shift: ShifterExecuteMsg,
        depth_shift: ShifterExecuteMsg,

        insurance_fund_withdraw: ControllerExecuteMsg,
        set_market_enabled: ControllerExecuteMsg,
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
            insurance_fund_withdraw: execute_insurance_fund_withdraw(),
            set_market_enabled: execute_set_market_enabled(),
        };
        let json_str = serde_json::to_string_pretty(&example_msgs).unwrap();
        let mut file = File::create("./execute_msg.json").unwrap();
        assert!(file.write_all(json_str.as_bytes()).is_ok());
    }

    pub fn execute_open_position() -> NBExecuteMsg {
        NBExecuteMsg::OpenPosition {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
            is_long: true,
            quote_amount: Uint128::from(100u128),
            leverage: Decimal::from_str("5").unwrap(),
            base_amount_limit: Uint128::from(1000u128),
        }
    }

    pub fn execute_close_position() -> NBExecuteMsg {
        NBExecuteMsg::ClosePosition {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
        }
    }

    pub fn execute_add_margin() -> NBExecuteMsg {
        NBExecuteMsg::AddMargin {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
            margin: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
        }
    }

    pub fn execute_remove_margin() -> NBExecuteMsg {
        NBExecuteMsg::RemoveMargin {
            sender: DUMMY_ADDR.to_string(),
            pair: DUMMY_PAIR.to_string(),
            margin: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
        }
    }

    pub fn execute_multi_liquidate() -> NBExecuteMsg {
        NBExecuteMsg::MultiLiquidate {
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

    pub fn execute_donate_to_insurance_fund() -> NBExecuteMsg {
        NBExecuteMsg::DonateToInsuranceFund {
            sender: DUMMY_ADDR.to_string(),
            donation: Coin {
                denom: "uusd".to_string(),
                amount: Uint128::from(100u128),
            },
        }
    }

    pub fn execute_peg_shift() -> ShifterExecuteMsg {
        ShifterExecuteMsg::PegShift {
            pair: DUMMY_PAIR.to_string(),
            peg_mult: dec_420(),
        }
    }

    pub fn execute_depth_shift() -> ShifterExecuteMsg {
        ShifterExecuteMsg::DepthShift {
            pair: DUMMY_PAIR.to_string(),
            depth_mult: dec_420(),
        }
    }

    pub fn execute_insurance_fund_withdraw() -> ControllerExecuteMsg {
        ControllerExecuteMsg::InsuranceFundWithdraw {
            amount: Uint256::from(420u128),
            to: DUMMY_ADDR_2.to_string(),
        }
    }

    pub fn execute_set_market_enabled() -> ControllerExecuteMsg {
        ControllerExecuteMsg::SetMarketEnabled {
            pair: DUMMY_PAIR.to_string(),
            enabled: true,
        }
    }

    pub fn liquidation_args() -> LiquidationArgs {
        LiquidationArgs {
            pair: DUMMY_PAIR.to_string(),
            trader: "trader".to_string(),
        }
    }
}
