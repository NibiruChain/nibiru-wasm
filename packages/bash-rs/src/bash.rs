use std::process;

use serde::Serialize;

use std::io::{BufRead, BufReader};
use std::process::{Child, Stdio};
use std::thread;

use crate::errors::BashError;

/// Runs a shell command
pub fn run_bash(bash_code: String) -> Result<BashOutput, BashError> {
    let mut bash_cmd = build_bash_cmd(bash_code.clone());
    let output = bash_cmd.output().expect("Failed to execute command");

    match output.status.success() {
        true => Ok(BashOutput::new(bash_code, output)),
        false => {
            let error_json = serde_json::to_string_pretty(&BashOutput {
                status: output
                    .status
                    .code()
                    .expect("Failed to parse cmd status code"),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                cmd: bash_code.clone(),
            })
            .expect("Failed to parse bash command output as string");
            Err(BashError::BashCmdFailed { err: error_json })
        }
    }
}

fn build_bash_cmd(bash_code: String) -> std::process::Command {
    let mut bash_cmd = std::process::Command::new("bash");
    bash_cmd.arg("-c").arg(bash_code);
    bash_cmd
}

/// run_cmd_print: Runs a command and prints its output.
pub fn run_bash_and_print(bash_code: String) -> Result<(), BashError> {
    let mut bash_cmd = build_bash_cmd(bash_code.clone());
    let mut child: Child = bash_cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Capture stdout and stderr
    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");
    let stderr = child
        .stderr
        .take()
        .expect("child did not have a handle to stderr");

    // Spawn a thread to handle stdout
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line.expect("Could not read line from stdout"));
        }
    });

    // Spawn a thread to handle stderr
    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            eprintln!("{}", line.expect("Could not read line from stderr"));
        }
    });

    // Wait for threads to finish
    stdout_thread
        .join()
        .expect("The stdout thread has panicked");
    stderr_thread
        .join()
        .expect("The stderr thread has panicked");

    // Wait for the process to finish
    let status = child.wait()?;

    match status.success() {
        true => Ok(()),
        false => Err(BashError::General { msg: bash_code }),
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct BashOutput {
    pub status: i32,
    pub cmd: String,
    pub stdout: String,
    pub stderr: String,
}

impl BashOutput {
    pub fn new(cmd: String, output: process::Output) -> Self {
        let mut bash_cmd_out: BashOutput = output.into();
        bash_cmd_out.cmd = cmd;
        bash_cmd_out
    }
}

impl From<process::Output> for BashOutput {
    fn from(output: process::Output) -> Self {
        BashOutput {
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
    let out = run_bash(format!(
        r#"
if which {bin} >/dev/null; then
    echo '{bin} is present'
else
    echo '{bin} is not installed'
fi"#,
    ))
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
