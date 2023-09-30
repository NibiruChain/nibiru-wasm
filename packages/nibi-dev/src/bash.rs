use std::process;

use serde::Serialize;

use crate::errors::BashError;

/// Runs a shell command
pub fn run_bash(cmd: String) -> Result<BashCommandOutput, BashError> {
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(cmd.clone())
        .output()
        .expect("Failed to execute command");

    match output.status.success() {
        true => Ok(BashCommandOutput::new(cmd, output)),
        false => {
            let error_json = serde_json::to_string_pretty(&BashCommandOutput {
                status: output
                    .status
                    .code()
                    .expect("Failed to parse cmd status code"),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                cmd: cmd.clone(),
            })
            .expect("Failed to parse bash command output as string");
            Err(BashError::BashCmdFailed(cmd, error_json))
        }
    }
}

/// run_cmd_print: Runs a command and prints its output.
pub fn run_bash_and_print(cmd: String) -> Result<BashCommandOutput, BashError> {
    let out = run_bash(cmd)?;
    if !out.stdout.is_empty() {
        println!("{}", out.stdout);
    }
    if !out.stderr.is_empty() {
        println!("{}", out.stderr);
    }
    Ok(out)
}

#[derive(Serialize, Debug, Clone)]
pub struct BashCommandOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
    pub cmd: String,
}

impl BashCommandOutput {
    pub fn new(cmd: String, output: process::Output) -> Self {
        let mut bash_cmd_out: BashCommandOutput = output.into();
        bash_cmd_out.cmd = cmd;
        bash_cmd_out
    }
}

impl From<process::Output> for BashCommandOutput {
    fn from(output: process::Output) -> Self {
        BashCommandOutput {
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            cmd: "".to_string(),
        }
    }
}

/// which_ok: True if the executable, "bin", is in the $PATH.
pub fn which_ok(bin: &str) -> bool {
    let err_msg_string = format!("failed to run 'which {}'", bin);
    let err_msg = err_msg_string.as_str();
    let out = run_bash(
        [
            format!("if which {} >/dev/null; then", bin),
            format!("    echo '{} is present'", bin),
            "else".to_string(),
            format!("    echo '{} is not installed'", bin),
            "fi".to_string(),
        ]
        .join("\n"),
    )
    .expect(err_msg);
    out.stdout.contains("is present")
}

#[cfg(test)]
mod tests {
    use super::{run_bash, which_ok};

    #[test]
    fn test_run_cmd() {
        let cmds: Vec<&str> = vec!["ls -l", "jq"];
        cmds.iter().for_each(|cmd| {
            let out = run_bash(cmd.to_string());
            assert!(out.is_ok())
        });
    }

    #[test]
    fn test_bad_command() {
        let cmd = "somecmd that doesn't exist";
        let out = run_bash(cmd.into());
        assert!(out.is_err());
    }

    #[test]
    fn test_redirection() {
        let content_str = "hello";
        let cmd = format!("echo {} > temp-test.txt", content_str);
        let mut out = run_bash(cmd);
        assert!(out.is_ok());
        let output = std::fs::read_to_string("temp-test.txt").unwrap();
        assert_eq!(output, format!("{}\n", content_str));

        // cleanup
        out = run_bash("rm temp-test.txt".into());
        assert!(out.is_ok())
    }

    #[test]
    fn test_have_binary() {
        let mut cmd = "ls";
        let mut have = which_ok(cmd);
        assert!(have);

        cmd = "cat";
        have = which_ok(cmd);
        assert!(have);

        cmd = "notabinary";
        have = which_ok(cmd);
        assert!(!have);
    }
}
