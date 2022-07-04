use crate::add_coins;
use crate::error::ContractError;
use crate::events::{new_incentives_program_event, new_program_funding};
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{funding, EpochInfo, Funding, Program, EPOCH_INFO, FUNDING_ID, LAST_EPOCH_PROCESSED, LOCKUP_ADDR, PROGRAMS, PROGRAMS_ID, WITHDRAWALS};
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Timestamp, Uint128, Uint64,
};
use cw_storage_plus::Bound;
use lockup::msgs::QueryMsg as LockupQueryMsg;
use lockup::state::{Lock, NOT_UNLOCKING_BLOCK_IDENTIFIER};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Duration;

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ProgramFunding { program_id: id } => to_binary(
            &funding()
                .idx
                .pay_from_epoch
                .sub_prefix(id)
                .range(deps.storage, None, None, Order::Ascending)
                .map(|res| -> Funding { res.unwrap().1 })
                .collect::<Vec<Funding>>(),
        ),
        QueryMsg::EpochInfo { program_id, epoch_number } => {
            to_binary(&EPOCH_INFO.load(deps.storage, (program_id, epoch_number))?)
        }
    }
}

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PROGRAMS_ID.save(deps.storage, &0).unwrap();
    FUNDING_ID.save(deps.storage, &0).unwrap();
    LOCKUP_ADDR
        .save(deps.storage, &msg.lockup_contract_address)
        .unwrap(); // TODO(mercilex): maybe check if addr exist in wasm

    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    return match msg {
        ExecuteMsg::CreateProgram {
            denom,
            epochs,
            epoch_block_duration,
            min_lockup_blocks,
            start_block,
        } => execute_create_program(
            deps,
            env,
            info,
            denom,
            epochs,
            epoch_block_duration,
            min_lockup_blocks,
            start_block,
        ),

        ExecuteMsg::FundProgram { id } => execute_fund_program(deps, env, info, id),

        ExecuteMsg::ProcessEpoch { id } => execute_process_epoch(deps, env, info, id),

        ExecuteMsg::WithdrawRewards { id } => execute_withdraw_rewards(deps, env, info, id),

        _ => Err(ContractError::NotImplemented),
    };
}

fn execute_fund_program(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: u64,
) -> Result<Response, ContractError> {
    if info.funds.len() == 0 {
        return Err(ContractError::FundsRequired);
    }

    // assert program exists
    let program = PROGRAMS.load(deps.storage, program_id)?;
    // equality because epoch processing can be triggered before funding
    if program.end_block <= env.block.height {
        return Err(ContractError::ProgramFinished(program_id));
    }

    // prepare response event before
    // so we can avoid to copy funds
    // due to funds being moved
    let response = Response::new().add_event(new_program_funding(program_id, &info.funds));

    // update funding associated with the program id for this block
    let mut epochs_to_pay = (program.end_block - env.block.height) / program.epoch_duration;
    // if it's 0 it means that the division was < 1
    // which means this funding applies only to the last epoch
    if epochs_to_pay == 0 {
        epochs_to_pay = 1
    }
    let pay_from_epoch = program.epochs - epochs_to_pay + 1;

    for coin in info.funds {
        let funding_id = FUNDING_ID
            .update(deps.storage, |id| -> StdResult<_> { Ok(id + 1) })
            .unwrap();

        funding()
            .save(
                deps.storage,
                funding_id,
                &Funding {
                    id: funding_id,
                    program_id,
                    pay_from_epoch,
                    denom: coin.denom,
                    initial_amount: coin.amount,
                    to_pay_each_epoch: coin.amount / Uint128::from(epochs_to_pay),
                },
            )
            .unwrap();
    }

    Ok(response)
}

fn execute_create_program(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    epochs: u64,
    epoch_duration_blocks: u64,
    min_lockup_blocks: u64,
    start_block: u64,
) -> Result<Response, ContractError> {
    let id = PROGRAMS_ID
        .update(deps.storage, |id| -> StdResult<u64> { return Ok(id + 1) })
        .unwrap();

    let program = Program {
        id,
        epochs,
        epoch_duration: epoch_duration_blocks,
        min_lockup_duration_blocks: min_lockup_blocks,
        lockup_denom: denom,
        start_block,
        end_block: env.block.height + (epochs * epoch_duration_blocks),
    };
    PROGRAMS.save(deps.storage, id, &program)?;

    Ok(Response::new().add_event(new_incentives_program_event(&program)))
}

fn execute_withdraw_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: u64,
) -> Result<Response, ContractError> {
    // fetch the program
    let program = PROGRAMS.load(deps.storage, program_id)?;

    // find epochs that need to be paid for this addr
    let epochs_to_pay = EPOCH_INFO
        .prefix(program_id)
        .range(deps.storage,
        Some(
            WITHDRAWALS.load(deps.storage, (program_id, info.sender.clone()))
                .map_or_else(|_| -> Bound<'_, _> {
                    Bound::inclusive(0 as u64)
                }, |last_withdrawal| -> Bound<'_, _> {
                    Bound::exclusive(last_withdrawal)
                })
        ),
        None,
        Order::Ascending)
        .map(|epoch| -> EpochInfo {
            epoch.unwrap().1
        })
        .collect::<Vec<EpochInfo>>();

        let mut last_paid_epoch = 0;
        let mut to_distribute: Vec<Coin> = vec![];
        for epoch in epochs_to_pay {
            // query lockup to check if the user has some qualified locks
            let epoch_qualified_locks: Vec<Lock> = deps.querier.query_wasm_smart(
                    LOCKUP_ADDR.load(deps.storage).unwrap(),
                    &lockup::msgs::QueryMsg::LocksByDenomAndAddressUnlockingAfter {
                        denom: program.lockup_denom.clone(),
                        unlocking_after: epoch.for_coins_unlocking_after,
                        address: info.sender.clone(),
                    })?;

            println!("{:?}", epoch_qualified_locks.clone());

            // get the total amount of locked coins
            let qualified_locked_amount = epoch_qualified_locks
                .iter()
                .map(|lock| -> Uint128 { lock.coin.amount})
                .sum::<Uint128>();

            // now for each coin to distribute
            // we compute the weight of the
            // of the sender in the qualified locks
            println!("qualified locked amount: {:?}, {:?}", qualified_locked_amount, env.block.height);
            let user_ownership_ratio = qualified_locked_amount / epoch.total_locked;
            println!("ownership ratio: {:?}", user_ownership_ratio.to_string());
            for coin in epoch.to_distribute {
                add_coins(&mut to_distribute, Coin{
                    denom: coin.denom,
                    amount: coin.amount * user_ownership_ratio,
                })
            }

            last_paid_epoch = epoch.epoch_identifier;
        }

    if last_paid_epoch == 0 {
        return Err(ContractError::NothingToWithdraw("".to_string()))
    }

    WITHDRAWALS.save(deps.storage, (program_id, info.sender.clone()), &last_paid_epoch).unwrap();


    println!("distributing: {:?}", &to_distribute);
    Ok(Response::new().add_message(
        BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: to_distribute,
        }
    ))

}

fn execute_process_epoch(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    program_id: u64,
) -> Result<Response, ContractError> {
    let program = PROGRAMS.load(deps.storage, program_id)?;
    let epoch_to_process =
        LAST_EPOCH_PROCESSED.update(deps.storage, program_id, |epoch| -> StdResult<u64> {
            // if this is the first time processing the epoch
            // then the identifier is 1. Then we increase each time
            // by 1 sequentially.
            Ok(epoch.unwrap_or_default() + 1)
        })?;

    if epoch_to_process > program.epochs {
        return Err(ContractError::EpochOutOfBounds(
            epoch_to_process,
            program_id,
        ));
    }

    // we get at which block the epoch should be processed
    let epoch_process_block = program.start_block + (epoch_to_process * program.epoch_duration);
    // epochs can be processed after the end of the epoch process block
    if env.block.height <= epoch_process_block {
        return Err(ContractError::EpochProcessBlock(
            epoch_to_process,
            program_id,
            epoch_process_block,
        ));
    }

    let lockup_qualification_block = epoch_process_block + program.min_lockup_duration_blocks;

    // the concept of epoch processing is quite straight forward
    // we just need to know which addresses hadn't withdrawn
    // the lockup denom before a certain block height
    let locks: Vec<Lock> = deps.querier.query_wasm_smart(
        LOCKUP_ADDR.load(deps.storage).unwrap(),
        &LockupQueryMsg::LocksByDenomUnlockingAfter {
            denom: program.lockup_denom,
            unlocking_after: lockup_qualification_block,
        },
    )?;

    // identify how much is the total locked
    let total_locked = locks
        .iter()
        .map(|lock| -> Uint128 { lock.coin.amount })
        .sum::<Uint128>();

    // then we identify how many coins we need to pay
    // based on the program funding

    let mut to_distribute = vec![];

    for funds in funding()
        .idx
        .pay_from_epoch
        .sub_prefix(program_id)
        .range(
            deps.storage,
            Some(Bound::inclusive((epoch_to_process, 0))),
            None,
            Order::Ascending,
        )
        .map(|funds| -> Coin {
            funds
                .map(|x| -> Coin {
                    Coin {
                        denom: x.1.denom,
                        amount: x.1.to_pay_each_epoch,
                    }
                })
                .unwrap()
        })
    {
        add_coins(&mut to_distribute, funds)
    }

    EPOCH_INFO.save(
        deps.storage,
        (program_id, epoch_to_process),
        &EpochInfo {
            epoch_identifier: epoch_to_process,
            for_coins_unlocking_after: lockup_qualification_block,
            to_distribute,
            total_locked,
        },
    )?;

    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_program() {}
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
