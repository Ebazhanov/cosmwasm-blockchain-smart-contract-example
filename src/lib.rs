use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

mod contract;
pub mod error;
pub mod msg;
#[cfg(test)]
pub mod multitest;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg.counter, msg.minimal_donation)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    match msg {
        Value {} => to_binary(&contract::query::value(deps)?),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> Result<Response, ContractError> {
    use msg::ExecMsg::*;

    match msg {
        Donate {} => contract::exec::donate(deps, info).map_err(ContractError::from),
        Withdraw {} => contract::exec::withdraw(deps, env, info),
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::{coin, coins, Addr, Coin, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::msg::{ExecMsg, QueryMsg, ValueResp};

    use super::*;

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    const ATOM: &str = "atom";

    #[test]
    fn unauthorized_withdraw() {
        let owner = Addr::unchecked("owner");
        let member = Addr::unchecked("member");

        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        let err = app
            .execute_contract(
                member.clone(),
                contract_addr.clone(),
                &ExecMsg::Withdraw {},
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {
                owner: owner.into()
            },
            err.downcast().unwrap()
        );
    }
}
