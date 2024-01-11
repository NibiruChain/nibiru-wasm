//! wasm_out.rs: Compiles smart contracts into .wasm bytecode using
//! rust-optimizer, creating determinstic builds and reducing gas usage.
//!
//! Ref: https://github.com/CosmWasm/rust-optimizer
//!
//! For now, this re-implements the functionality of the neighboring bash script.

use std::{env, fs};

use bash_rs::run_bash_and_print;
use clap::{command, Arg, ArgAction, ArgMatches, Command};

pub const IMAGE_VERSION: &str = "0.14.0";
pub const APP_NAME: &str = "wasm_out";

fn main() -> anyhow::Result<()> {
    // let args: Vec<String> = std::env::args().collect();
    ensure_expected_pwd()?;

    let cli_cmd: Command = new_cmd();
    let matches = cli_cmd.clone().try_get_matches();

    let exec_path = CmdExecPath::new(matches)?;
    let exec_path = exec_path.exec(cli_cmd).map_err(|err| {
        println!("❌ Compilation failed.\n");
        err
    })?;

    #[allow(clippy::single_match)]
    match exec_path {
        CmdExecPath::All => {
            println!("🔥 Compiled all smart contracts successfully.")
        }
        _ => {}
    }

    Ok(())
}

pub fn new_cmd() -> clap::Command {
    let about = "Compiles CosmWasm smart contracts to WebAssembly bytecode for use in production.";
    command!(APP_NAME).about(about).subcommand(
        Command::new("all")
            .about("Compile all smart contracts")
            .alias("a"),
    )
}

#[allow(dead_code)]
fn new_arg_dry_run() -> Arg {
    Arg::new("dry-run")
        .short('d')
        .long("dry-run")
        .help("Skip actual runs. Used for testing only.")
        .action(ArgAction::SetTrue)
}

#[derive(Debug)]
pub enum WasmCompilationScheme {
    /// Compile a workspace of smart contracts with the cosmwasm/workspace-optimizer
    /// Docker image.
    All,
}

impl WasmCompilationScheme {
    pub fn docker_image(&self) -> &'static str {
        match self {
            WasmCompilationScheme::All => "workspace-optimizer",
        }
    }

    /// Compiles CosmWasm smart contracts to WebAssembly bytecode (.wasm) using
    /// rust-optimizer. Optimize to reduce gas.
    ///
    /// Ref: https://github.com/CosmWasm/rust-optimizer
    pub fn run(&self) -> anyhow::Result<()> {
        self.print_run_preamble();
        let scheme = self;
        let current_dir = std::env::current_dir()
            .expect("Failed to get path to current directory");
        let current_dir_name = current_dir
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or(anyhow::anyhow!(
                "expected to parse current directory name"
            ))?;

        let image = scheme.docker_image();
        let cmd = format!(
        "docker run --rm -v {current_dir:?}:/code \
        --mount type=volume,source={current_dir_name}_cache,target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        cosmwasm/{image}:{IMAGE_VERSION}",
    );
        // current_dir, current_dir_name, image, IMAGE_VERSION

        run_bash_and_print(&cmd).map_err(|bash_err| anyhow::anyhow!(bash_err))
    }

    fn print_run_preamble(&self) {
        match self {
            WasmCompilationScheme::All => println!(
                "Running rust-optimizer for all contracts (CosmWasm/workspace-optimizer)"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CmdExecPath {
    All,
    Help,
    Unknown,
}

impl CmdExecPath {
    pub fn new(
        matches: Result<ArgMatches, clap::error::Error>,
    ) -> anyhow::Result<Self> {
        let mut exec_path = CmdExecPath::Unknown;
        match matches {
            Ok(matches) => {
                exec_path = parse_arg_matches(&matches)?;
            }
            Err(err) => match err.kind() {
                clap::error::ErrorKind::DisplayHelp => {}
                _ => println!("DEBUG: {err}"),
            },
        }
        Ok(exec_path)
    }

    pub fn exec(self, mut cli_cmd: Command) -> anyhow::Result<CmdExecPath> {
        match self {
            CmdExecPath::All => {
                WasmCompilationScheme::All.run()?;
            }
            CmdExecPath::Help => {}
            CmdExecPath::Unknown => {
                cli_cmd.print_help()?;
            }
        }

        Ok(self)
    }
}

pub fn parse_arg_matches(matches: &ArgMatches) -> anyhow::Result<CmdExecPath> {
    if matches.try_get_one::<bool>("help")?.is_some() {
        return Ok(CmdExecPath::Help);
    }

    #[allow(clippy::single_match)]
    match matches.subcommand_name() {
        Some("all") => return Ok(CmdExecPath::All),
        _ => {}
    }

    Ok(CmdExecPath::Unknown)
}

pub fn ensure_expected_pwd() -> anyhow::Result<()> {
    // Get the current executable's directory.
    let current_dir = env::current_dir()?;

    // Check if we're inside the 'scripts' directory.
    if current_dir.ends_with("scripts") {
        // Change to the parent directory.
        let parent_dir =
            current_dir.parent().expect("failed to parse parent dir");
        env::set_current_dir(parent_dir)?;
    }

    // Now, verify that "scripts" is a direct child of the current directory.
    let current_dir = env::current_dir()?;
    let scripts_path = current_dir.join("scripts");
    if !scripts_path.is_dir() {
        return Err(anyhow::anyhow!(
            "'scripts' directory not found as a direct child of {}. Current children: {:?}",
            &current_dir.display(),
            fs::read_dir(&current_dir)?
                .map(|entry| entry.map(|e| e.file_name()))
                .collect::<Result<Vec<_>, _>>()?
        ));
    }
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    use std::sync::Mutex;

    lazy_static::lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    pub type TestResult = anyhow::Result<()>;

    fn run_test_cases(test_cases: Vec<(Vec<&str>, CmdExecPath)>) -> TestResult {
        for (args, want) in test_cases {
            let cli_cmd = new_cmd();
            let matches_result = cli_cmd.try_get_matches_from(args.clone());
            let exec_path = CmdExecPath::new(matches_result)?;
            assert_eq!(exec_path, want, "Failed on args: {args:?}");
        }
        Ok(())
    }

    #[test]
    pub fn api_exec_paths() -> TestResult {
        let test_cases: Vec<(Vec<&str>, CmdExecPath)> = vec![
            (vec![APP_NAME, "all"], CmdExecPath::All),
            (vec![APP_NAME, "foobar", "bat"], CmdExecPath::Unknown),
            (vec![APP_NAME, "all", "--help"], CmdExecPath::Unknown),
            (vec![APP_NAME, "-h"], CmdExecPath::Unknown),
        ];

        run_test_cases(test_cases)?;

        Ok(())
    }

    #[test]
    pub fn new_cmd_debug_assert() -> TestResult {
        let cmd = new_cmd();
        cmd.debug_assert();
        Ok(())
    }

    /// Test to ensure that when the current working directory is the parent of
    /// 'scripts', the function `ensure_expected_pwd` does not alter the
    /// directory.
    #[test]
    fn test_ensure_expected_pwd_inside_scripts_parent_dir() -> TestResult {
        let _guard = TEST_MUTEX.lock().unwrap();
        let dir = tempdir()?;
        let repo_path = dir.path().join("repo");
        fs::create_dir(&repo_path)?;

        // Create 'scripts' directory inside 'repo'
        let scripts_path = repo_path.join("scripts");
        fs::create_dir(scripts_path)?;

        env::set_current_dir(&repo_path)?;
        ensure_expected_pwd()?;

        assert_eq!(env::current_dir()?, repo_path);
        Ok(())
    }

    /// Test to ensure that when the current working directory is 'scripts', the
    /// function `ensure_expected_pwd` correctly moves the directory to its
    /// parent.
    #[test]
    fn test_ensure_expected_pwd_inside_scripts_dir() -> TestResult {
        let _guard = TEST_MUTEX.lock().unwrap();
        let dir = tempdir()?;
        let repo_path = dir.path().join("repo");
        fs::create_dir(&repo_path)?;
        let scripts_path = repo_path.join("scripts");
        fs::create_dir(&scripts_path)?;

        env::set_current_dir(&scripts_path)?;
        ensure_expected_pwd()?;

        assert_eq!(env::current_dir()?, repo_path);
        Ok(())
    }

    /// Test to verify that if the current directory is not 'scripts' or its parent,
    /// the function `ensure_expected_pwd` correctly returns an error.
    #[test]
    fn test_invalid_executable_path() -> TestResult {
        let _guard = TEST_MUTEX.lock().unwrap();
        let dir = tempdir()?;
        let bad_path = dir.path().join("not-scripts-dir");
        fs::create_dir(&bad_path)?;
        env::set_current_dir(&bad_path)?;

        let result = ensure_expected_pwd();
        assert!(result.is_err(), "{bad_path:?}, {dir:?}");
        Ok(())
    }
}
