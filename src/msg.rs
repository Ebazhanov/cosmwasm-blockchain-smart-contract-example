use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Parent {
    pub addr: String,
    pub donating_period: u64,
    pub part: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
    pub minimal_donation: Coin,
    pub parent: Option<Parent>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
}

#[cw_serde]
pub enum ExecMsg {
    Donate {},
    Withdraw {},
}

#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
    pub parent: Option<Parent>,
}
