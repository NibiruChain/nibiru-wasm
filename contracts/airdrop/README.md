# Airdrop Contract

## Overview

The airdrop contract is used to distribute tokens to a list of addresses. An instance of a contract represents a campaign. The contract is initialized with a campaign id, a campaign name, a campaign description, an owner (the deployer of the contract), a list of managers, and funds which become the unallocated amount.

The token allocation amount starts unallocated and eventually gets allocated to users by the owner and managers.

The contract owner and managers can allocate/reward users with tokens by calling the `reward_users` function. The `reward_users` function takes a list of addresses and amounts. The total reward amount must be less than the unallocated token amount of the contract.

## Withdraw

Only the contract owner can withdraw from the contract (not the managers). The `withdraw` exists to withdraw any leftover tokens after the campaign has ended. There is no check for if the total outstanding reward amount is greater than the amount of funds left in the contract. `withdraw` should only be called after the campaign ends because it could leave the contract in a state where it cannot fulfill a user's outstanding reward amount. Additional funds can be sent to the contract to reverse the withdrawal.
