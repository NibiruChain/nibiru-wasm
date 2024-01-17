use std::process;

use serde::Serialize;

use std::io::{BufRead, BufReader};
use std::process::{Child, Stdio};
use std::thread;

use crate::errors::BashError;

/// Runs a shell command
pub fn run_bash(bash_code: &str) -> Result<BashOutput, BashError> {
    let mut bash_cmd = build_bash_cmd(bash_code);
    let output = bash_cmd.output().expect("Failed to execute command");

    match output.status.success() {
        true => Ok(BashOutput::new(bash_code.to_string(), output)),
        false => {
            let error_json = serde_json::to_string_pretty(&BashOutput {
                status: output
                    .status
                    .code()
                    .expect("Failed to parse cmd status code"),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                cmd: bash_code.to_string(),
            })
            .expect("Failed to parse bash command output as string");
            Err(BashError::BashCmdFailed { err: error_json })
        }
    }
}

/// build_bash_cmd: Constructs a `process::Command` corresponding to the given `bash_code`.
pub fn build_bash_cmd(bash_code: &str) -> std::process::Command {
    let mut bash_cmd = std::process::Command::new("bash");
    bash_cmd.arg("-c").arg(bash_code);
    bash_cmd
}

/// run_bash_and_print: Runs a command and prints its output as it is read into
/// the buffer. This differs from the default behavior of `run_bash`, which puts
/// all of the stdout and stderr into aggregate strings at the end of the command
/// execution.
pub fn run_bash_and_print(bash_code: &str) -> Result<(), BashError> {
    let mut bash_cmd = build_bash_cmd(bash_code);
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
        false => Err(BashError::General {
            msg: bash_code.to_string(),
        }),
    }
}

/// BashOutput: Output of a bash command.
#[derive(Serialize, Debug, Clone)]
pub struct BashOutput {
    /// status: Exist status code. 0 for success. Everything else means failure.
    pub status: i32,
    /// cmd: The input bash command that was run.
    pub cmd: String,
    /// stdout: Standard output stream. This includes the main data that the
    /// command sends to the terminal with successful operation.
    pub stdout: String,
    /// stderr: Standard error output stream. Stderr captures error messages,
    /// warnings, and other diagnostic information.
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
    let out = run_bash(&format!(
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

/// which_ok_assert: Validate that the given binary is in PATH. Error if not.
///
/// ### Examples
/// ```
/// use bash_rs::which_ok_assert;
///
/// let result = which_ok_assert("ls");
/// assert!(result.is_ok());
///
/// let result = which_ok_assert("non_existent_binary");
/// assert!(result.is_err());
/// ```
pub fn which_ok_assert(bin: &str) -> Result<(), BashError> {
    if !which_ok(bin) {
        return Err(BashError::WhichBinNotPresent {
            bin: bin.to_string(),
        });
    }
    Ok(())
}

/// Run multiple bash commands in succession without returning the output.
pub fn run_bash_multi(cmds: Vec<&str>) -> Result<Vec<BashOutput>, BashError> {
    let mut outs: Vec<BashOutput> = Vec::new();
    for cmd in cmds {
        let out = run_bash(cmd)?;
        outs.push(out);
    }
    Ok(outs)
}

#[cfg(test)]
mod tests {
    use super::{run_bash, which_ok};

    #[test]
    fn test_run_cmd() {
        let cmds: Vec<&str> = vec!["ls -l", "jq"];
        cmds.iter().for_each(|cmd| {
            let out = run_bash(cmd);
            assert!(out.is_ok())
        });
    }

    #[test]
    fn test_bad_command() {
        let cmd = "somecmd that doesn't exist";
        let out = run_bash(cmd);
        assert!(out.is_err());
    }

    #[test]
    fn test_redirection() {
        let content_str = "hello";
        let cmd = format!("echo {} > temp-test.txt", content_str);
        let mut out = run_bash(&cmd);
        assert!(out.is_ok());
        let output = std::fs::read_to_string("temp-test.txt").unwrap();
        assert_eq!(output, format!("{}\n", content_str));

        // cleanup
        out = run_bash("rm temp-test.txt");
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
