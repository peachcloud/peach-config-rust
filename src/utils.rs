use log::{debug, info};
use snafu::ResultExt;
use std::process::{Command, Output};

use crate::constants::CONF;
use crate::error::PeachConfigError;
use crate::error::{CmdIoError, CmdParseOutputError};

/// Utility function which takes in a vector of &str and executes them as a bash command.
/// This function is intended to make scripted bash via rust more ergonomic.
///
/// The first item in the vector is used as the command,
/// and the following items, if supplied, are used as arguments for the command.
///
/// Returns a std::process::Output if successful and a PeachConfigError otherwise.
pub fn cmd(args: &[&str]) -> Result<Output, PeachConfigError> {
    info!("command: {:?}", args);
    let output = Command::new(args[0])
        .args(&args[1..args.len()])
        .output()
        .context(CmdIoError {
            command: format!("{:?}", args),
        })?;
    debug!("output: {:?}", output);
    if output.status.success() {
        Ok(output)
    } else {
        let err_msg = String::from_utf8(output.stderr).expect("failed to read stderr");
        Err(PeachConfigError::CmdError {
            msg: err_msg,
            command: format!("{:?}", args),
        })
    }
}

/// Utility function which calls calls cmd (above) but converts the Output to a String
/// before returning.
pub fn get_output(args: &[&str]) -> Result<String, PeachConfigError> {
    let output = cmd(args)?;
    let std_out = std::str::from_utf8(&output.stdout).context(CmdParseOutputError {
        command: format!("{:?}", args),
    })?;
    let mut std_out = std_out.to_string();
    if std_out.ends_with('\n') {
        std_out.pop();
    }
    Ok(std_out)
}

/// Takes in a relative path from the conf dir and returns the absolute path to the file
pub fn conf(path: &str) -> String {
    let full_path = format!("{}/{}", CONF, path);
    full_path
}

/// Creates a linux group with the given name if it doesn't already exist
pub fn create_group_if_doesnt_exist(group: &str) -> Result<(), PeachConfigError> {
    let output = Command::new("getent")
        .arg("group")
        .arg(group)
        .output()
        .context(CmdIoError {
            command: format!("getent group {}", group),
        })?;
    if output.status.success() {
        // then group already exists
        Ok(())
    } else {
        // otherwise create group
        cmd(&["/usr/sbin/groupadd", group])?;
        Ok(())
    }
}

/// Creates a linux user with the given username if it doesn't already exist
pub fn does_user_exist(user: &str) -> Result<bool, PeachConfigError> {
    let output = Command::new("getent")
        .arg("passwd")
        .arg(user)
        .output()
        .context(CmdIoError {
            command: format!("getent passwd {}", user),
        })?;
    if output.status.success() {
        // then user already exists
        Ok(true)
    } else {
        // otherwise user does not exist
        Ok(false)
    }
}
