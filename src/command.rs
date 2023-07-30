use anyhow::Result;
use std::process::Command;
use std::process::ExitStatus;

#[derive(Debug)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}

/// No exec_dir with [command].
pub fn command_no_dir(program: &str, args: Vec<&str>) -> Result<CommandResult> {
    command("", program, args)
}

/// Exec command with exec_dir,program,args.
///
/// # Examples
///
/// ```
/// use tracing::info;
///
/// let res = cool_common::command("./", "cargo", vec!["--version"]).unwrap();
/// info!("{:?}", res);
/// ```
pub fn command(exec_dir: &str, program: &str, args: Vec<&str>) -> Result<CommandResult> {
    let ref mut command = Command::new(program);
    if !exec_dir.is_empty() {
        command.current_dir(exec_dir);
    }
    let output = command.args(args).output()?;
    let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let status = output.status;

    // windows
    if cfg!(windows) {
        // windows 使用 GB18030
        use encoding::all::GB18030;
        use encoding::DecoderTrap;
        use encoding::Encoding;
        stdout = GB18030
            .decode(&output.stdout, DecoderTrap::Strict)
            .map_err(|e| anyhow::format_err!(e))?;
        stderr = GB18030
            .decode(&output.stderr, DecoderTrap::Strict)
            .map_err(|e| anyhow::format_err!(e))?;
    }
    Ok(CommandResult {
        stdout,
        stderr,
        status,
    })
}
