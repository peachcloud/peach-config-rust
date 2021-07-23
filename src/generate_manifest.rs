use std::fs;
use snafu::ResultExt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use regex::{Regex};

use crate::error::{PeachConfigError, FileWriteError, FileReadError};
use crate::utils::{get_output};
use crate::constants::HARDWARE_CONFIG_FILE;
use crate::RtcOption;

/// returns a HashMap<String, String> of all the peach-packages which are currently installed
/// mapped to their version number e.g. { "peach-probe": "1.2.0", "peach-network": "1.4.0" }
pub fn get_currently_installed_microservices() -> Result<HashMap<String, String>, PeachConfigError> {

    // gets a list of all packages currently installed with dpkg
    let packages = get_output(&["dpkg", "-l"])?;

    // this regex matches packages which contain the word peach in them
    // and has two match groups
    // 1. the first match group gets the package name
    // 2. the second match group gets the version number of the package
    let re: Regex = Regex::new(r"\S+\s+(\S*peach\S+)\s+(\S+).*\n").unwrap();

    // the following iterator, iterates through the captures matched via the regex
    // and for each capture, creates a value in the hash map,
    //  which maps the name of the package, to its version number
    // e.g. { "peach-probe": "1.2.0", "peach-network": "1.4.0" }
    let peach_packages: HashMap<String, String> = re.captures_iter(&packages).filter_map(|cap| {
        let groups = (cap.get(1), cap.get(2));
        match groups {
            (Some(package), Some(version)) => {
                Some((package.as_str().to_string(), version.as_str().to_string()))
            },
            _ => None,
        }
    }).collect();

    // finally the hashmap of packages and version numbers is returned
    Ok(peach_packages)
}

/// output form of manifest
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    // packages is a map of {package_name: version}
    packages: HashMap<String, String>,
    hardware: Option<HardwareConfig>
}

/// the form that hardware configs are saved in when peach-config setup runs successfully
#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareConfig {
    // packages is a map of {package_name: version}
    i2c: bool,
    rtc: Option<RtcOption>
}

/// log which hardware settings were configured to a .json file
pub fn save_hardware_config(i2c: bool, rtc: Option<RtcOption>) -> Result<HardwareConfig, PeachConfigError> {

    let hardware_config = HardwareConfig { i2c, rtc };

    let json_str = serde_json::to_string(&hardware_config)?;

    fs::write(HARDWARE_CONFIG_FILE, json_str).context(FileWriteError {
        file: HARDWARE_CONFIG_FILE.to_string(),
    })?;

    Ok(hardware_config)
}

/// load the hardware configs that were saved from the last successful run of peach-config setup
fn load_hardware_config() -> Result<Option<HardwareConfig>, PeachConfigError> {
    // if there is no hardware_config, return None
    let hardware_config_exists = std::path::Path::new(HARDWARE_CONFIG_FILE).exists();
    if !hardware_config_exists {
        Ok(None)
    }
    // otherwise we load hardware_config from json
    else {
        let contents = fs::read_to_string(HARDWARE_CONFIG_FILE).context(FileReadError {
            file: HARDWARE_CONFIG_FILE.to_string(),
        })?;
        let hardware_config: HardwareConfig = serde_json::from_str(&contents)?;
        Ok(Some(hardware_config))
    }
}

/// outputs a Manifest in json form to stdout
/// which contains the currently installed peach packages
/// as well as the hardware configuration of the last run of peach-config setup
pub fn generate_manifest() -> Result<(), PeachConfigError> {
    let packages = get_currently_installed_microservices()?;
    let hardware_config_option = load_hardware_config()?;
    let manifest = Manifest {
        packages,
        hardware: hardware_config_option,
    };
    let output = serde_json::to_string(&manifest)?;
    println!("{}", output);
    Ok(())
}

