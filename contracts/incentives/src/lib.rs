extern crate core;

use cosmwasm_std::Coin;

mod contract;
mod error;
mod events;
mod msgs;
mod state;
#[cfg(test)]
mod testing;

fn add_coins(coins: &mut Vec<Coin>, coin: Coin) {
    for i in 0..coins.len() {
        if coins[i].denom == coin.denom {
            coins[i].amount += coin.amount;
            return;
        }
    }
    coins.push(coin)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
