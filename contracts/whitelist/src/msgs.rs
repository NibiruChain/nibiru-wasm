use std::collections::HashSet;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InitMsg {
    pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddMember { address: String },
    RemoveMember { address: String },
}

#[cw_serde]
pub enum QueryMsg {
    IsMember { address: String },
    Members {},
}

#[cw_serde]
pub struct IsMemberResponse {
    pub is_member: bool,
}

#[cw_serde]
pub struct MembersResponse {
    pub admin: String,
    pub members: HashSet<String>,
}
