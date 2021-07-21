use log::info;
use std::process::{Command, Output};
use snafu::ResultExt;

use crate::error::PeachConfigError;
use crate::error::{CmdIoError, CmdParseOutputError};
use crate::constants::CONF;


pub fn cmd(args: &[&str]) -> Result<Output, PeachConfigError> {
    info!("command: {:?}", args);
    let output = Command::new(args[0]).args(&args[1 .. args.len()]).output().context(CmdIoError {
            command: format!("{:?}", args),
        })?;
    info!("output: {:?}", output);
    if output.status.success() {
        Ok(output)
    }
    else {
        let err_msg = String::from_utf8(output.stderr).expect("failed to read sterr");
        Err(PeachConfigError::CmdError{ msg: err_msg, command: format!("{:?}", args) })
    }
}

pub fn get_output(args: &[&str]) -> Result<String, PeachConfigError> {
    let output = cmd(args)?;
    let std_out = std::str::from_utf8(&output.stdout).context(
        CmdParseOutputError { command: format!("{:?}", args) }
    )?;
    let mut std_out = std_out.to_string();
    if std_out.ends_with('\n') {
        std_out.pop();
    }
    Ok(std_out)
}

/// takes in a relative path from the conf dir and returns the full path
pub fn conf(path: &str) -> String {
    let full_path = format!("{}/{}", CONF, path);
    full_path
}


pub fn create_group_if_doesnt_exist(group: &str) -> Result<(), PeachConfigError> {
    let output = get_output(&["getent", "group", group])?.to_string();
    if output.contains(group) {
        // then group already exists, just return Ok
        Ok(())
    } else {
        // otherwise create the group
        cmd(&["/usr/sbin/groupadd", group])?;
        Ok(())
    }
}


pub fn does_user_exist(user: &str) -> Result<bool, PeachConfigError> {
    let output = get_output(&["getent", "passwd", user])?.to_string();
    if output.contains(user) {
        // then group already exists, just return Ok
        Ok(true)
    } else {
        Ok(false)
    }
}