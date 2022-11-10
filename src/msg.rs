use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub counter: u64,
    pub minimal_donation: Coin,
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
