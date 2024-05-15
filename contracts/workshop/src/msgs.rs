use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(HelloResp)]
    HelloWorld {},
    #[returns(GetCountResp)]
    GetCount {},
}

#[cw_serde]
pub struct HelloResp {
    pub greeting: String,
}

#[cw_serde]
pub struct GetPriceResp {
    pub exchange_rate: String,
}

#[cw_serde]
pub struct GetCountResp {
    pub count: i32,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}
