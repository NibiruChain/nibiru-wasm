use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

/// Structure for the message that instantiates the smart contract.
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub managers: Vec<String>,
}

/// Enum respresenting message types for the execute entry point.
/// These express the different ways in which one can invoke the contract
/// and broadcast tx messages against it.
#[cw_serde]
pub enum ExecuteMsg {
    /// SetAutocompounderMode allows to set the autocompounder mode.
    /// If it's set to true, managers will be able to stake tokens, otherwise
    /// they won't be able to do so.
    SetAutocompounderMode { autocompounder_mode: bool },

    /// Withdraw allows to withdraw a given amount of tokens from the contract.
    /// The Withdraw message defines the tokens amount and the recipient address
    Withdraw { amount: Uint128, recipient: String },

    /// Unstake allows to unstake a given amount of tokens from a set of
    /// validators. The UnstakeMsgs defines the tokens amount and address
    /// of the validator.
    Unstake { unstake_msgs: Vec<UnstakeMsg> },

    /// UpdateManagers allows to update the list of managers.
    UpdateManagers { managers: Vec<String> },

    /// Manager functions

    /// Stake allows to stake a given amount of tokens to a set of validators.
    /// The StakeMsgs defines the share of tokens distributed and the validator
    /// to which the stake is made.
    Stake {
        amount: Uint128,
        stake_msgs: Vec<StakeMsg>,
    },
}

#[cw_serde]
pub struct UnstakeMsg {
    pub amount: Uint128,
    pub validator: String,
}

#[cw_serde]
pub struct StakeMsg {
    pub share: Uint128,
    pub validator: String,
}

/// Enum representing the message types for the query entry point.
#[cw_serde]
pub enum QueryMsg {
    AutocompounderMode {},
    AdminAndManagers {},
}
