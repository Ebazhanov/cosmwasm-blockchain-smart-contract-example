use cosmwasm_std::{coins, Addr, Coin, Empty};
use cw_multi_test::{App, Contract, ContractWrapper};

use crate::error::ContractError;
use crate::multitest::CountingContract;
use crate::{execute, instantiate, query};

fn counting_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

const ATOM: &str = "atom";

#[test]
fn query_value() {
    let mut app = App::default();
    let sender = Addr::unchecked("sender");

    let contract_id = app.store_code(counting_contract());
    let contract = CountingContract::instantiate(
        &mut app,
        contract_id,
        &sender,
        "Counting_contract",
        Coin::new(10, ATOM),
    )
    .unwrap();
    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 0);
}

#[test]
fn donate() {
    let mut app = App::default();

    let sender = Addr::unchecked("sender");

    let contract_id = app.store_code(counting_contract());
    let contract = CountingContract::instantiate(
        &mut app,
        contract_id,
        &sender,
        "Counting_contract",
        Coin::new(10, ATOM),
    )
    .unwrap();
    contract.donate(&mut app, &sender, &[]).unwrap();
    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 0);
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
    let contract = CountingContract::instantiate(
        &mut app,
        contract_id,
        &sender,
        "Counting_contract",
        Coin::new(10, ATOM),
    )
    .unwrap();
    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();

    let resp = contract.query_value(&app).unwrap();
    assert_eq!(resp.value, 1);
    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        coins(10, ATOM)
    )
}

#[test]
fn withdraw() {
    let owner = Addr::unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    });

    let contract_id = app.store_code(counting_contract());

    let contract = CountingContract::instantiate(
        &mut app,
        contract_id,
        &owner,
        "Counting_contract",
        Coin::new(10, ATOM),
    )
    .unwrap();
    contract
        .donate(&mut app, &sender, &coins(10, ATOM))
        .unwrap();

    contract.withdraw(&mut app, &owner).unwrap();

    assert_eq!(
        app.wrap().query_all_balances(owner).unwrap(),
        coins(10, "atom")
    );
    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
    assert_eq!(
        app.wrap().query_all_balances(contract.addr()).unwrap(),
        vec![]
    );
}

#[test]
fn unauthorized_withdraw() {
    let owner = Addr::unchecked("owner");
    let member = Addr::unchecked("member");

    let mut app = App::default();

    let contract_id = app.store_code(counting_contract());

    let contract = CountingContract::instantiate(
        &mut app,
        contract_id,
        &owner,
        "Counting_contract",
        Coin::new(10, ATOM),
    )
    .unwrap();

    let err = contract.withdraw(&mut app, &member).unwrap_err();
    assert_eq!(
        err,
        ContractError::Unauthorized {
            owner: owner.into()
        },
    );
}
