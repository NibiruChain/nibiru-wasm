use crate::error::ContractError;
use crate::msgs::{
    CurrentPrice, ExecuteMsg, InstantiateMsg, Market, QueryMarketsResponse, QueryMsg,
    QueryOraclesResponse, QueryPriceResponse, QueryPricesResponse, QueryRawPriceResponse, SudoMsg,
};
use crate::state::{
    CurrentTWAP, PostedPrice, ACTIVE_PAIRS, CURRENT_PRICES, CURRENT_TWAP, ORACLE_PAIR_WHITELIST,
    RAW_PRICES,
};
use crate::AssetPair;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Decimal, Deps, DepsMut, Empty, Env, Event, MessageInfo,
    Order, Response, StdResult, Storage, Uint128,
};
use cw_storage_plus::Bound;
use schemars::_serde_json::to_string;
use std::collections::HashSet;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Price { pair_id } => to_binary(&QueryPriceResponse {
            current_price: CurrentPrice {
                pair_id: pair_id.clone(),
                price: CURRENT_PRICES.load(deps.storage, pair_id.clone())?,
            },
        }),
        QueryMsg::Prices {} => to_binary(&QueryPricesResponse {
            current_prices: CURRENT_PRICES
                .range(deps.storage, None, None, Order::Ascending)
                .map(|r| -> CurrentPrice {
                    let (pair_id, price) = r.unwrap();
                    CurrentPrice { pair_id, price }
                })
                .collect::<Vec<CurrentPrice>>(),
        }),

        QueryMsg::RawPrices { pair_id } => to_binary(&QueryRawPriceResponse {
            raw_prices: RAW_PRICES
                .prefix(pair_id)
                .range(deps.storage, None, None, Order::Ascending)
                .map(|r| -> PostedPrice { r.unwrap().1 })
                .collect::<Vec<PostedPrice>>(),
        }),
        QueryMsg::Oracles {} => {
            let mut unique_oracles: HashSet<Addr> = HashSet::new();
            let oracles: Vec<Addr> = ORACLE_PAIR_WHITELIST
                .keys(deps.storage, None, None, Order::Ascending)
                .map(|r| -> Addr { r.unwrap().1 })
                .collect();

            for oracle in oracles {
                unique_oracles.insert(oracle);
            }

            to_binary(&QueryOraclesResponse {
                oracles: unique_oracles.into_iter().collect::<Vec<Addr>>(),
            })
        }
        QueryMsg::Markets {} => {
            let mut markets: Vec<Market> = vec![];
            let pairs: Vec<String> = ACTIVE_PAIRS
                .keys(deps.storage, None, None, Order::Ascending)
                .map(|r| -> String { r.unwrap() })
                .collect();

            for pair in pairs {
                let mut market = Market {
                    pair_id: pair.clone(),
                    oracles: ORACLE_PAIR_WHITELIST
                        .prefix(pair.clone())
                        .keys(deps.storage, None, None, Order::Ascending)
                        .map(|o| -> Addr { o.unwrap() })
                        .collect(),
                    active: true,
                };
                markets.push(market)
            }

            to_binary(&QueryMarketsResponse { markets })
        }
    }
}

pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    return match msg {
        SudoMsg::BeginBlock { .. } => {
            for asset_pair in ACTIVE_PAIRS
                .range(deps.storage, None, None, Order::Ascending)
                .map(|o| -> AssetPair { AssetPair::try_from(o.unwrap().0).unwrap() })
                .collect::<Vec<AssetPair>>()
            {
                // get raw prices
                let mut raw_prices: Vec<Decimal> = RAW_PRICES
                    .prefix(asset_pair.to_string())
                    .range(deps.storage, None, None, Order::Ascending)
                    .map(|o| -> PostedPrice { o.unwrap().1 })
                    .filter(|o| -> bool { return o.expiry > env.block.time })
                    .map(|o| -> Decimal { o.price })
                    .collect();

                let previous_price =
                    CURRENT_PRICES.may_load(deps.storage, asset_pair.to_string())?;

                if raw_prices.len() == 0 {
                    CURRENT_PRICES.remove(deps.storage, asset_pair.to_string());
                    return Err(ContractError::NoValidPrice);
                }

                // calculate median price
                let median_price = calculate_median_price(raw_prices);
                if previous_price.unwrap_or_default() != median_price {
                    // todo event emission
                }

                CURRENT_PRICES.save(deps.storage, asset_pair.to_string(), &median_price)?;

                // update twap

                // here we get current price, which we already do know
                // get current twap
                let current_twap = CURRENT_TWAP
                    .load(deps.storage, asset_pair.to_string())
                    .unwrap_or(CurrentTWAP {
                        pair_id: asset_pair.to_string(),
                        numerator: Decimal::zero(),
                        denominator: Decimal::zero(),
                        price: Decimal::zero(),
                    });

                let new_denominator = current_twap.denominator
                    + Decimal::new(Uint128::from(env.block.time.seconds()));
                let new_numerator = current_twap.numerator
                    + (median_price * Decimal::new(Uint128::from(env.block.time.seconds())));

                CURRENT_TWAP.save(
                    deps.storage,
                    asset_pair.to_string(),
                    &CurrentTWAP {
                        pair_id: asset_pair.to_string(),
                        numerator: new_numerator,
                        denominator: new_denominator,
                        price: new_numerator / new_denominator,
                    },
                )?;
            }
            Ok(Response::new())
        }
        SudoMsg::AddOracleProposal { oracles, pairs } => {
            for pair in pairs {
                AssetPair::try_from(pair.clone())?;
                for oracle in &oracles {
                    ORACLE_PAIR_WHITELIST.save(
                        deps.storage,
                        (pair.clone(), oracle),
                        &Empty::default(),
                    )?;
                }
            }

            Ok(Response::new())
        }
    };
}

fn calculate_median_price(mut prices: Vec<Decimal>) -> Decimal {
    let l = prices.len();
    if l == 0 {
        return prices[0];
    }

    prices.sort();

    if l % 2 == 0 {
        // calculate median
        let (p1, p2) = (prices[l / 2 - 1], prices[l / 2]);
        let sum = p1 + p2;
        return sum / Decimal::new(Uint128::new(2));
    }

    return prices[l / 2];
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    return match msg {
        ExecuteMsg::PostPrice {
            token0,
            token1,
            mut price,
            expiry,
        } => {
            let mut asset_pair = AssetPair::new(token0, token1);
            // check if whitelisted
            let whitelisted =
                ORACLE_PAIR_WHITELIST.has(deps.storage, (asset_pair.to_string(), &info.sender));
            let inverse_whitelist =
                ORACLE_PAIR_WHITELIST.has(deps.storage, (asset_pair.to_string(), &info.sender));

            if !whitelisted && !inverse_whitelist {
                return Err(ContractError::Unauthorized(info.sender));
            }

            if inverse_whitelist {
                price = Decimal::one() / price;
            }

            // we entered keeper.PostRawPrice

            if expiry < env.block.time {
                return Err(ContractError::Expired);
            }

            // we check for price being positive
            if price < Decimal::zero() {
                return Err(ContractError::NegativePrice);
            }

            if ACTIVE_PAIRS.has(deps.storage, asset_pair.inverse().to_string()) {
                asset_pair = asset_pair.inverse()
            }

            if !ORACLE_PAIR_WHITELIST.has(deps.storage, (asset_pair.to_string(), &info.sender)) {
                return Err(ContractError::Unauthorized(info.sender));
            }

            RAW_PRICES.save(
                deps.storage,
                (asset_pair.to_string(), &info.sender),
                &PostedPrice {
                    pair_id: asset_pair.to_string(),
                    oracle: info.sender.clone(),
                    price,
                    expiry,
                },
            )?;

            Ok(Response::new().add_event(
                Event::new("oracle_price_update")
                    .add_attribute("pair_id", asset_pair.to_string())
                    .add_attribute("oracle", info.sender.to_string())
                    .add_attribute("price", price.to_string())
                    .add_attribute("expiry", expiry.to_string()),
            ))
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {}
}
