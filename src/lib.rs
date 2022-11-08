use crate::msg::InstantiateMsg;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

mod contract;
pub mod msg;
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
) -> StdResult<Response> {
    use msg::ExecMsg::*;

    let _ = match msg {
        Donate {} => contract::exec::donate(deps, info),
        Withdraw {} => contract::exec::withdraw(deps, env, info),
    };
    Ok(Response::new())
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

    #[test]
    fn query_value() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: Coin::new(10, ATOM),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }

    const ATOM: &str = "atom";

    #[test]
    fn donate() {
        let mut app = App::default();

        let sender = Addr::unchecked("sender");

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                sender.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: Coin::new(10, ATOM),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(sender, contract_addr.clone(), &ExecMsg::Donate {}, &[])
            .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }

    #[test]
    fn donate_with_funds() {
        let sender = Addr::unchecked("sender");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap();
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, ATOM),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, ATOM),
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }

    #[test]
    fn withdraw() {
        let owner = Addr::unchecked("owner");
        let sender1 = Addr::unchecked("sender1");
        let sender2 = Addr::unchecked("sender2");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender1, coins(10, "atom"))
                .unwrap();
            router
                .bank
                .init_balance(storage, &sender2, coins(5, "atom"))
                .unwrap();
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, ATOM),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            sender1.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, ATOM),
        )
        .unwrap();

        app.execute_contract(
            sender2.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(5, ATOM),
        )
        .unwrap();

        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &ExecMsg::Withdraw {},
            &[],
        )
        .unwrap();

        assert_eq!(
            app.wrap().query_all_balances(owner).unwrap(),
            coins(15, ATOM)
        );
        assert_eq!(
            app.wrap().query_all_balances(contract_addr).unwrap(),
            vec![]
        );
        assert_eq!(app.wrap().query_all_balances(sender1).unwrap(), vec![]);
        assert_eq!(app.wrap().query_all_balances(sender2).unwrap(), vec![]);
    }
}
