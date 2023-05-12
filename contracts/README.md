# Contracts               <!-- omit in toc -->
- [shifter](#shifter)
    - [Contained Functionality](#contained-functionality)
    - [Entry Points](#entry-points)

---

# shifter 

"Shifter" is a simple contract that can be used to execute peg shift and
depth shifts in the x/perp module of Nibiru. The contract stores a whitelist
of addresses, managed by an admin. This whitelist design takes inspiration
from cw-plus/contracts/cw1-whitelist.

The contract initializes with an admin address and allows the admin to add
or remove addresses from the whitelist. Users can query whether an address
is whitelisted or not.

### Contained Functionality

1. Initialize the contract with an admin address.
2. Allow the admin to add or remove addresses from the whitelist.
3. Allow anyone to query if an address is on the whitelist.
4. Members of the whitelist set can execute permissioned calls on the Nibiru
   x/perp module for dynamic optimizations like peg shift and depth shift.

### Entry Points

- InitMsg: Initializes the contract with the admin address.
- ExecuteMsg: Enum for executing msgs
  - ExecuteMsg::DepthShift
  - ExecuteMsg::PegShift
  - ExecuteMsg::AddMember adds an address to the whitelist
  - ExecuteMsg::RemoveMember removes and address from the whitelist.
  - ExecuteMsg::ChangeAdmin lets the current admin set a new one.

---