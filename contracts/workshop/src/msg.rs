use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    HelloWorld {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HelloResp {
    pub greeting: String,
}
