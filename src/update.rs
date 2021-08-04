use crate::constants::SERVICES;
use crate::error::PeachConfigError;
use crate::utils::{cmd, get_output};
use crate::UpdateOpts;
use serde::{Deserialize, Serialize};

/// Parses update subcommand CLI arguments and calls correct methods.
///
/// If no options are passed, it runs a full update
/// - first updating peach-config
/// - and then re-running peach-config to update all the other microservices
///
/// # Arguments
///
/// * `opts` - an UpdateOpts object containing parsed CLI args
///
/// Any error results in a PeachConfigError, otherwise an Ok is returned.
pub fn update(opts: UpdateOpts) -> Result<(), PeachConfigError> {
    if opts.self_only {
        run_update_self()
    } else if opts.microservices {
        update_microservices()
    } else if opts.list {
        list_available_updates()
    }
    // otherwise no options were passed, and we do a full update:
    // - first updating peach-config
    // - and then re-running peach-config to update all the other microservices
    else {
        run_update_self()?;
        cmd(&["/usr/bin/peach-config", "update", "--microservices"])?;
        Ok(())
    }
}

/// Updates peach-config using apt-get
pub fn run_update_self() -> Result<(), PeachConfigError> {
    cmd(&["apt-get", "update"])?;
    cmd(&["apt-get", "install", "-y", "peach-config"])?;
    Ok(())
}

/// Installs all peach microservices or updates them to the latest version
/// except for peach-config
pub fn update_microservices() -> Result<(), PeachConfigError> {
    // update apt
    cmd(&["apt-get", "update"])?;
    // filter out peach-config from list of services
    let services_to_update: Vec<&str> = SERVICES
        .to_vec()
        .into_iter()
        .filter(|&x| x != "peach-config")
        .collect();

    // apt-get install all services
    let mut update_cmd = ["apt-get", "install", "-y"].to_vec();
    update_cmd.extend(services_to_update);
    cmd(&update_cmd)?;
    Ok(())
}

/// Output form of list_available_updates
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAvailableUpdatesOutput {
    // packages is a list of package names
    upgradeable: Vec<String>,
}

/// Checks if there are any PeachCloud updates available and displays them
pub fn list_available_updates() -> Result<(), PeachConfigError> {
    cmd(&["apt-get", "update"])?;
    let output = get_output(&["apt", "list", "--upgradable"])?;
    let lines = output.split('\n');
    // filter down to just lines which are one of the services
    let upgradeable: Vec<String> = lines
        .into_iter()
        .filter(|x| SERVICES.iter().any(|s| x.contains(s)))
        .map(|x| x.to_string())
        .collect();
    let list_available_updates_output = ListAvailableUpdatesOutput { upgradeable };
    let output = serde_json::to_string(&list_available_updates_output)?;
    println!("{}", output);
    Ok(())
}
