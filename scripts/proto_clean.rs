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

pub fn clean_file_imports(
    rust_proto_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Define a regular expression to match `super::` imports
    let re = regex::Regex::new(r"super::(?:super::)+[\w:]+")?;
    let content = std::fs::read_to_string(rust_proto_path)?;
    // Replace all matches in the file content using the provided function
    let updated_content = re.replace_all(&content, |caps: &regex::Captures| {
        let matched = &caps[0]; // Get the entire matched string
        match super_import_to_clean(matched) {
            Ok(cleaned) => cleaned,
            Err(err) => {
                eprintln!("Error cleaning import '{}': {:?}", matched, err);
                matched.to_string() // If there's an error, leave the import unchanged
            }
        }
    });

    Ok(updated_content.to_string())
}

/// Runs clean_file_imports and writes new contents back to the input file.
pub fn clean_file_imports_inplace(
    rust_proto_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match clean_file_imports(rust_proto_path) {
        Ok(content) => {
            std::fs::write(rust_proto_path, content)?;
            Ok(())
        }
        Err(err) => Err(err),
    }
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

pub static PROTO_PATH: &str = "../nibiru-std/src/proto";

#[cfg(test)]
mod tests {
    use std::fs;

    #[derive(Debug, PartialEq)]
    pub struct TestCase {
        input: &'static str,
        want_err: bool,
        want_out: Option<&'static str>,
    }

    #[test]
    fn test_super_import_to_clean() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                input: "::super::super::super::bank::foo",
                want_err: false,
                want_out: Some("crate::proto::cosmos::bank::foo"),
            },
            TestCase {
                input: "::super::tendermint::xyz",
                want_err: false,
                want_out: Some("crate::proto::tendermint::xyz"),
            },
            TestCase {
                input: "abcxyz",
                want_err: true,
                want_out: None,
            },
        ];

        for (i, test_case) in test_cases.iter().enumerate() {
            let result = super::super_import_to_clean(test_case.input);

            // Check if the result is an error as expected
            if test_case.want_err {
                assert!(result.is_err(), "Test case {} failed", i);
            } else {
                assert!(result.is_ok(), "Test case {} failed", i);
                // Check the expected output
                if let Ok(cleaned) = result {
                    assert_eq!(
                        cleaned,
                        test_case.want_out.unwrap(),
                        "Test case {} failed",
                        i
                    );
                }
            }
        }
    }

    #[test]
    fn fixture_proto_clean() {
        let dirty_path = "test/fixture_proto_dirty.rs";
        let result = super::clean_file_imports(dirty_path);
        assert!(result.is_ok());

        // let _ = fs::write(clean_path, result.as_ref().unwrap());
        let clean_path = "test/fixture_proto_clean.rs";
        let want_result = fs::read_to_string(clean_path);
        assert!(want_result.is_ok());

        assert_eq!(result.unwrap(), want_result.unwrap());
    }
}