use cw_storage_plus::Item;
use std::collections::BTreeSet;

/// ACCEPTED_DENOMS: Defines the set of denominations that can be converted to
/// and from NUSD.
pub const ACCEPTED_DENOMS: Item<BTreeSet<String>> = Item::new("accepted_denoms");
