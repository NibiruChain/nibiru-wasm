#[cfg(test)]
mod integration_test {
    use crate::contract::{execute, instantiate, query};
    use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::state::{EpochInfo, Funding};

    use cosmwasm_std::{from_binary, Addr, Coin};
    use cw_multi_test::{App, BankSudo, Contract, ContractWrapper, Executor};

    const ROOT: &str = "root";
    const INCENTIVES: &str = "contract1";
    const LOCKUP: &str = "contract0";

    fn create_program(
        app: &mut App,
        denom: String,
        epochs: u64,
        epoch_block_duration: u64,
        min_lockup_blocks: u64,
    ) {
        app.execute_contract(
            Addr::unchecked(ROOT),
            Addr::unchecked(INCENTIVES),
            &ExecuteMsg::CreateProgram {
                denom,
                epochs,
                epoch_block_duration,
                min_lockup_blocks,
            },
            &[],
        )
        .unwrap();
    }

    fn fund_program(
        app: &mut App,
        _program_id: u64,
        coins: &[Coin],
    ) -> Vec<Coin> {
        app.execute_contract(
            Addr::unchecked(ROOT),
            Addr::unchecked(INCENTIVES),
            &ExecuteMsg::FundProgram { id: 1 },
            coins,
        )
        .unwrap();

        app.wrap().query_all_balances(ROOT).unwrap()
    }

    fn mint(app: &mut App, to: &Addr, coins: &[Coin]) {
        app.sudo(
            BankSudo::Mint {
                to_address: to.to_string(),
                amount: coins.to_vec(),
            }
            .into(),
        )
        .unwrap();
    }

    fn mint_and_lock(app: &mut App, user: &Addr, coins: &[Coin], blocks: u64) {
        mint(app, user, coins);
        // we make alice lock some atoms
        app.execute_contract(
            user.clone(),
            Addr::unchecked(LOCKUP),
            &lockup::msgs::ExecuteMsg::Lock { blocks },
            coins,
        )
        .unwrap();
    }

    fn withdraw_rewards(
        app: &mut App,
        user: &Addr,
        _program_id: u64,
    ) -> Vec<Coin> {
        app.execute_contract(
            user.clone(),
            Addr::unchecked(INCENTIVES),
            &crate::msgs::ExecuteMsg::WithdrawRewards { id: 1 },
            &[],
        )
        .unwrap();

        app.wrap().query_all_balances(user).unwrap()
    }

    fn process_epoch(app: &mut App, _program_id: u64) -> EpochInfo {
        from_binary::<EpochInfo>(
            &app.execute_contract(
                Addr::unchecked(ROOT),
                Addr::unchecked(INCENTIVES),
                &ExecuteMsg::ProcessEpoch { id: 1 },
                &[],
            )
            .unwrap()
            .data
            .unwrap(),
        )
        .unwrap()
    }

    fn app() -> App {
        let mut app = App::default();
        // note don't break the order otherwise contracts will have different addresses
        // which renders the const in the module useless TODO: maybe do better.

        let lockup_contract = Box::new(ContractWrapper::new(
            lockup::contract::execute,
            lockup::contract::instantiate,
            lockup::contract::query,
        ));
        let code = app.store_code(lockup_contract);
        app.instantiate_contract(
            code,
            Addr::unchecked(ROOT),
            &lockup::msgs::InstantiateMsg {},
            &[],
            "lockup",
            None,
        )
        .unwrap();

        let incentives_contract =
            Box::new(ContractWrapper::new(execute, instantiate, query));
        let code = app.store_code(incentives_contract);
        app.instantiate_contract(
            code,
            Addr::unchecked(ROOT),
            &InstantiateMsg {
                lockup_contract_address: Addr::unchecked(LOCKUP),
            },
            &[],
            "incentives",
            None,
        )
        .unwrap();

        app
    }

    #[test]
    fn flow() {
        let mut app = app();
        let _lockup_addr = Addr::unchecked(LOCKUP);
        let _incentives_addr = Addr::unchecked(INCENTIVES);

        let alice = Addr::unchecked("alice");
        let bob = Addr::unchecked("bob");

        // mint coins
        mint(
            &mut app,
            &Addr::unchecked(ROOT),
            &[
                Coin::new(1_000_000, "ATOM"),
                Coin::new(1_000_000, "OSMO"),
                Coin::new(1_000_000, "LUNA"),
            ],
        );

        // we make alice lock some lp coins
        mint_and_lock(&mut app, &alice, &[Coin::new(100, "NIBI_LP")], 100);

        // now we create a new incentives program
        create_program(&mut app, "NIBI_LP".to_string(), 5, 5, 50);

        // now we fund the incentives program
        let balance = fund_program(&mut app, 1, &[Coin::new(1_000, "ATOM")]);

        println!("{:?}", balance,);

        let funding: Vec<Funding> = app
            .wrap()
            .query_wasm_smart(
                INCENTIVES,
                &QueryMsg::ProgramFunding { program_id: 1 },
            )
            .unwrap();
        println!("{:?}", funding);

        app.update_block(|block| {
            block.height += 6 // shift +1 because epoch can be processed after the epoch block
        });

        let epoch_info = process_epoch(&mut app, 1);

        println!("{:?}", epoch_info);

        // withdraw rewards for alice at epoch 1
        let alice_rewards = withdraw_rewards(&mut app, &alice, 1);

        // expected: 200ATOM coins
        assert_eq!(vec![Coin::new(200, "ATOM")], alice_rewards,);

        // add bob lock
        mint_and_lock(&mut app, &bob, &[Coin::new(200, "NIBI_LP")], 300);

        // bob qualifies for next epoch
        app.update_block(|block| {
            block.height += 5;
        });

        // process epoch
        let epoch_info = process_epoch(&mut app, 1);
        println!("{:?}", epoch_info);

        // withdraw rewards for alice at epoch 2
        let alice_balance = withdraw_rewards(&mut app, &alice, 1);

        // expected: 200 + 0.33*200 ATOM coins
        assert_eq!(vec![Coin::new(200 + 66, "ATOM")], alice_balance,);

        // withdraw rewards for bob at epoch 2
        let bob_balance = withdraw_rewards(&mut app, &bob, 1);

        // expected: 200 * 0.66666666 ATOM
        assert_eq!(vec![Coin::new(133, "ATOM")], bob_balance,);

        app.update_block(|block| block.height += 1);

        // add more funding
        fund_program(&mut app, 1, &[Coin::new(1000, "OSMO")]);

        // go to epoch 3 block and process it
        app.update_block(|block| block.height += 4);
        let epoch_info = process_epoch(&mut app, 1);
        println!("{:?}", epoch_info);

        // go to epoch 4 block and process it
        app.update_block(|block| block.height += 5);
        let epoch_info = process_epoch(&mut app, 1);
        println!("{:?}", epoch_info);

        // withdraw rewards for alice at epoch 3-4
        let alice_balance = withdraw_rewards(&mut app, &alice, 1);

        // withdraw rewards for bob at epoch 3-4
        let bob_balance = withdraw_rewards(&mut app, &bob, 1);

        assert_eq!(
            vec![Coin::new(399, "ATOM"), Coin::new(442, "OSMO")],
            bob_balance,
        );

        assert_eq!(
            vec![Coin::new(398, "ATOM"), Coin::new(220, "OSMO")],
            alice_balance,
        );

        // create a new funding for last epoch
        fund_program(&mut app, 1, &[Coin::new(1000, "OSMO")]);
        // fast forward to epoch 5
        app.update_block(|block| block.height += 5);

        // process epoch 5
        app.update_block(|block| block.height += 5);
        let epoch_info = process_epoch(&mut app, 1);
        println!("{:?}", epoch_info);

        // finalize distribution
        let alice_balance = withdraw_rewards(&mut app, &alice, 1);

        assert_eq!(
            vec![Coin::new(464, "ATOM"), Coin::new(664, "OSMO")],
            alice_balance,
        );

        let bob_balance = withdraw_rewards(&mut app, &bob, 1);
        assert_eq!(
            vec![Coin::new(532, "ATOM"), Coin::new(1330, "OSMO")],
            bob_balance,
        );
    }
}
