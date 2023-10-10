use std::{fs, io::Write, process};

use crate::{
    bash::{self, BashCommandOutput},
    errors::MsigError,
    multi::MsigSigner,
};

pub static PUBKEY_JSON_FILEPATH: &str = "msmem-pubkeys.json";
pub static KEY_TYPE_MULTI: &str = "multi";

pub fn load_pubkeys_json(
    pubkey_json_file: &str,
) -> Result<Vec<MsigSigner>, serde_json::Error> {
    let pubkeys_json: String = fs::read_to_string(pubkey_json_file)
        .unwrap_or_else(|_err| {
            let mut file = fs::File::create(pubkey_json_file)
                .expect("Failed to create pubkeys.json");
            file.write_all(b"[]").expect("Failed to write to file");
            String::from("[]")
        });

    return MsigSigner::from_str(pubkeys_json.as_str());
}

/// add_signers_keyring: Iterate through sig_signers, using the pubkey
/// and name for each one to execute
/// ```bash
/// nibid keys add msmem-{name} --pubkey='{pubkey}'
/// ```
pub fn add_signers_to_keyring(
    sig_signers: Vec<MsigSigner>,
) -> Result<Vec<BashCommandOutput>, MsigError> {
    let mut out: Vec<BashCommandOutput> = [].to_vec();
    for signer in sig_signers {
        let cmd: String = "echo \"y\" | ".to_owned()
            + &format!(
                "nibid keys add {} --pubkey='{}'",
                signer.gen_key_name(),
                signer.pubkey.as_cmd_arg(),
            )
            .to_string();
        // For each one, use the pubkey and name to execute
        match bash::run_cmd(cmd.clone()) {
            Ok(cmd_out) => out.push(BashCommandOutput::new(cmd, cmd_out)),
            Err(err) => {
                if err.to_string().contains(".address: no such file or") {
                    continue;
                } else {
                    return Err(MsigError::BashCmdFailed { err });
                }
            }
        }
    }
    Ok(out)
}

pub static MNEMONIC_GUARD_CREAM: &str = "guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host";
pub static ADDR_GUARD_CREAM: &str =
    "nibi1zaavvzxez0elundtn32qnk9lkm8kmcsz44g7xl";

pub fn add_guard_cream_to_keyring() -> Result<(), MsigError> {
    // Value we'll use as the key name for guard cream
    let key_name: &str = "val-gc";

    let cmd_keys_list: String = "nibid keys list --output=json".to_string();
    let keys_list_raw: process::Output =
        bash::run_cmd(cmd_keys_list.clone()).expect("Failed to query keyring");
    let std_buffers: Vec<Vec<u8>> =
        vec![keys_list_raw.stdout, keys_list_raw.stderr];
    let records_found: bool = !std_buffers.iter().any(|stdbuffer| {
        String::from_utf8_lossy(stdbuffer).contains("No records were found")
    });

    // Delete any keys that overlap with the "guard cream" account
    // We do this to guarantee consistent behavior over consecutive runs.
    let keys_list: Vec<MsigSigner> = if records_found {
        MsigSigner::from_str(&String::from_utf8_lossy(
            &bash::run_cmd(format!("{} | jq", cmd_keys_list))
                .expect("Failed to parse keys list output with jq")
                .stdout,
        ))
        .expect("unable to parse keys list json")
    } else {
        Vec::new()
    };

    let gc_names: Vec<String> = keys_list
        .iter()
        .filter(|&signer| {
            signer.clone().address.unwrap_or("".to_string()) == ADDR_GUARD_CREAM
        })
        .cloned()
        .map(|signer| signer.name)
        .collect();
    gc_names.iter().for_each(|key_name| {
        // Delete the key. It may already be empty, so we ignore the error
        let _ = bash::run_cmd(format!(
            "nibid keys delete {} --force --yes",
            key_name
        ));
    });

    clear_keys().unwrap();

    bash::run_cmd(format!(
        r#"echo "{}" | nibid keys add {} --recover --keyring-backend=test"#,
        MNEMONIC_GUARD_CREAM, key_name,
    ))?;
    Ok(())
}

pub fn clear_keys() -> Result<process::Output, MsigError> {
    let blockchain_binary = "nibid";

    let home_dir = home::home_dir();
    let home_path = match home_dir {
        Some(path) => path.to_string_lossy().into_owned(),
        None => {
            return Err(MsigError::General {
                err_msg: "No home dir",
            })
        }
    };

    let clear_cmd = format!(
        "rm -f {}/.{}/keyring-test/*.info",
        home_path, blockchain_binary,
    );

    bash::run_cmd(clear_cmd).map_err(|e| e.into())
}

/// Adds a multisig to the keyring named "generated"
pub fn generate_msig(sig_signers: Vec<MsigSigner>, threshold: u8) {
    let mut gen_key_names: Vec<String> = vec![];
    for signer in sig_signers {
        if signer.key_type == "multi" {
            continue;
        }
        gen_key_names.push(signer.gen_key_name())
    }

    let gen_key_names_arg: String = gen_key_names.join(",");

    bash::run_cmd(
        [
            r#"echo "y" |"#.to_string(),
            "nibid keys add generated".to_string(),
            format!("--multisig-threshold={}", threshold),
            format!("--multisig=\"{}\"", gen_key_names_arg),
        ]
        .join(" "),
    )
    .expect("failed to create multisig");

    bash::run_cmd("nibid keys show generated".to_owned())
        .expect("failed to show generated multisig");
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::multi::MsigSigner;
    use std::sync::Mutex;

    pub fn assert_res<T, E>() {}

    lazy_static::lazy_static! {
        /// The `lazy_static` macro allows for the creation of lazily-evaluated
        /// statics in Rust. Statics have a few constraints:
        ///
        /// 1. They have the `'static` lifetime, meaning they live for the
        /// entire dduration of the program.
        /// 2. Their type must be known at compile time, and they must be
        ///   initialized with constant expressions.
        ///
        /// Due to these constraints, complex initializations (like that of a
        /// `Mutex`) cannot be done in static declarations. `lazy_static`
        /// overcomes this by performing init at runtime when the static is
        /// accessed for the first time.
        ///
        /// In our case, this is when `TEST_RESOURCE_MUTEX` is accessed for
        /// the first time.
        pub static ref TEST_RESOURCE_MUTEX: Mutex<()> = Mutex::new(());
    }

    // TODO test generate_msig

    #[test]
    fn suite_test_api() {
        let _guard = TEST_RESOURCE_MUTEX.lock().unwrap();
        test_add_guard_cream();
        test_add_signers();
    }

    fn test_add_guard_cream() {
        clear_keys().unwrap();
        add_guard_cream_to_keyring().unwrap()
    }

    fn test_add_signers() {
        let pubkey_json_file: &str = "test/testdata_pubkeys.json";
        let sig_signers: Vec<MsigSigner> = load_pubkeys_json(pubkey_json_file)
            .expect("Failed to load pubkeys json");
        assert_eq!(sig_signers.len(), 7);

        let _outs =
            add_signers_to_keyring(sig_signers).expect("failed to add signers");

        // generate_msig(sig_signers, 2);
    }
}
