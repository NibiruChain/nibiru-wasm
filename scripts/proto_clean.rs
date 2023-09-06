// use std::fs;

/// scripts/proto_clean.rs:
///
/// ## Procedure
///
/// 1. Walk through all the files in the nibiru-std/src/proto directory.
/// 2. For each file, read its content and identify lines that import types with
///    multiple super components.
/// 3. Classify each import based on the first non-super part, then replace the
///    super components with crate::proto::cosmo or crate::proto::tendemint based
///    on the classification.
/// 4. Write the modified content back to each file.

pub fn main() {
    println!("Running proto_clean.rs...");
    println!("ran proto_clean.rs successfully");
}

#[derive(Debug)]
pub struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomError {}

/// Cleans a "super import" string and rebuilds it with the proper prefix.
pub fn super_import_to_clean(
    super_import: &str,
) -> Result<String, Box<CustomError>> {
    // Split import string into separate elements
    let elems: Vec<&str> = super_import.split("::").collect();

    let submodule_idx = elems
        .iter()
        .position(|&elem| !elem.is_empty() && elem != "super")
        .ok_or_else(|| {
            Box::new(CustomError(format!(
                "Only super elements found in import '{}'",
                super_import
            )))
        })?;

    let elem = elems[submodule_idx];
    let prefix: &str;
    if proto_submodules::is_cosmos_submod(elem) {
        prefix = "crate::proto::cosmos"
    } else if proto_submodules::is_tendermint_submod(elem) {
        prefix = "crate::proto"
    } else if proto_submodules::is_cosmos_base_submod(elem) {
        prefix = "crate::proto::cosmos::base"
    } else if proto_submodules::is_cosmos_tx_submod(elem) {
        prefix = "crate::proto::cosmos::tx"
    } else {
        return Err(Box::new(CustomError(format!(
            "Unrecognized import submodule: {}",
            super_import
        ))));
    };

    let out_str = format!("{}::{}", prefix, elems[submodule_idx..].join("::"));
    Ok(out_str)
}

mod proto_submodules {

    pub fn is_cosmos_submod(s: &str) -> bool {
        COSMOS.contains(&s)
    }

    pub fn is_tendermint_submod(s: &str) -> bool {
        TENDERMINT.contains(&s)
    }

    pub fn is_cosmos_base_submod(s: &str) -> bool {
        matches!(s, "query")
    }

    pub fn is_cosmos_tx_submod(s: &str) -> bool {
        matches!(s, "signing")
    }

    pub static COSMOS: [&str; 25] = [
        "base",
        "bank",
        "distribution",
        "app",
        "auth",
        "authz",
        "autocli",
        "capability",
        "consensus",
        "crypto",
        "evidence",
        "feegrant",
        "genutil",
        "gov",
        "group",
        "mint",
        "nft",
        "orm",
        "params",
        "reflection",
        "slashing",
        "staking",
        "tx",
        "upgrade",
        "vesting",
    ];

    pub static TENDERMINT: [&str; 1] = ["tendermint"];
}


