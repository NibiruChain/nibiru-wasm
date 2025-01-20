# contracts/broker-bank

Account abstration to enable funds to be held and sent to a whitelisted set of
accounts (`TO_ADDRS`). Bank transfers can only be called by "operators", and the
funds can only be withdrawn by the contract owner.