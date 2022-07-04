use crate::contract::{execute, instantiate, query};
use crate::msgs::{InstantiateMsg, QueryMsg};
use cosmwasm_std::testing::{mock_env, MockApi};
use cosmwasm_std::{Addr, Coin, Empty};
use cw_multi_test::{App, BankKeeper, BankSudo, Contract, ContractWrapper, Executor};
use crate::state::{EpochInfo, Funding};

fn mock_app() -> App {
    App::default()
}

fn incentives_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

fn deploy_incentives(app: &mut App, lockup_addr: &Addr) -> Addr {
    let contract = Box::new(ContractWrapper::new(execute, instantiate, query));

    let code = app.store_code(contract);
    app.instantiate_contract(
        code,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            lockup_contract_address: lockup_addr.clone(),
        },
        &[],
        "incentives",
        None,
    )
    .unwrap()
}

fn deploy_lockup(app: &mut App) -> Addr {
    let contract = Box::new(ContractWrapper::new(
        lockup::contract::execute,
        lockup::contract::instantiate,
        lockup::contract::query,
    ));

    let code = app.store_code(contract);

    app.instantiate_contract(
        code,
        Addr::unchecked("owner"),
        &lockup::msgs::InstantiateMsg {},
        &[],
        "lockup",
        None,
    )
    .unwrap()
}

#[test]
fn flow() {
    let mut app = mock_app();
    let owner = Addr::unchecked("owner");
    let alice = Addr::unchecked("alice");
    let bob = Addr::unchecked("bob");

    // mint coins
    app.sudo(
        BankSudo::Mint {
            to_address: alice.to_string(),
            amount: vec![Coin::new(1_000_000, "NIBI_LP")],
        }
        .into(),
    )
    .unwrap();
    app.sudo(
        BankSudo::Mint {
            to_address: owner.to_string(),
            amount: vec![Coin::new(1_000_000, "ATOM")],
        }
        .into(),
    )
    .unwrap();

    let lockup_addr = deploy_lockup(&mut app);
    let incentives_addr = deploy_incentives(&mut app, &lockup_addr);

    // we make alice lock some atoms
    app.execute_contract(
        alice.clone(),
        lockup_addr.clone(),
        &lockup::msgs::ExecuteMsg::Lock { blocks: 100 },
        &[Coin::new(100, "NIBI_LP")],
    )
    .unwrap();

    // now we create a new incentives program
    app.execute_contract(
        owner.clone(),
        incentives_addr.clone(),
        &crate::msgs::ExecuteMsg::CreateProgram {
            denom: "NIBI_LP".to_string(),
            epochs: 5,
            epoch_block_duration: 5,
            min_lockup_blocks: 50,
            start_block: app.block_info().height,
        },
        &[],
    )
    .unwrap();

    // now we fund the incentives program
    app.execute_contract(
        owner.clone(),
        incentives_addr.clone(),
        &crate::msgs::ExecuteMsg::FundProgram { id: 1 },
        &[Coin::new(1_000, "ATOM")],
    )
    .unwrap();

    println!(
        "{:?}",
        app.wrap()
            .query_all_balances(incentives_addr.clone())
            .unwrap()
    );

    let funding: Vec<Funding> = app.wrap().query_wasm_smart(incentives_addr.as_str(), &QueryMsg::ProgramFunding { program_id:1}).unwrap();
    println!("{:?}", funding);

    app.update_block(|block| {
        block.height = block.height + 6;
    });

    app.execute_contract(
        owner.clone(),
        incentives_addr.clone(),
        &crate::msgs::ExecuteMsg::ProcessEpoch {
            id: 1,
        },
        &[],
    ).unwrap();

    let epoch_info: EpochInfo = app.wrap().query_wasm_smart(incentives_addr.as_str(), &QueryMsg::EpochInfo{ program_id:1, epoch_number: 1}).unwrap();
    println!("{:?}", epoch_info)
}
