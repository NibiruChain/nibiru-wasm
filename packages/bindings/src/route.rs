use cosmwasm_schema::cw_serde;

/// Routes here refer to groups of operations that will be interpreted in
/// the x/wasm/binding packa . The idea here is to add
/// information on which module or group of modules a particular execute message  
/// belongs to.
#[cw_serde]
pub enum NibiruRoute {
    /// "perp" is the route corresponding to bindings for the x/perp module.
    Perp,

    /// "no_op" is a valid route that doesn't do anything. It's necessary for
    /// formatting in the custom Wasm execute handler.
    NoOp,
}
