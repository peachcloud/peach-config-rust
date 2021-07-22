use crate::constants::SERVICES;
use crate::error::PeachConfigError;
use crate::utils::{cmd, conf, get_output};
use crate::RtcOption;
use log::info;
use std::fs;

/// updates peach-config using apt-get
pub fn run_update_self() -> Result<(), PeachConfigError> {
    cmd(&["apt-get", "update"])?;
    cmd(&["apt-get", "install", "python3-peach-config"])?;
    Ok(())
}

/// installs all peach microservices or updates them to the latest version
/// except for peach-config
/// :return: None
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
    let mut update_cmd = ["apt-get", "install"].to_vec();
    update_cmd.extend(services_to_update);
    cmd(&update_cmd)?;
    Ok(())
}

//
//pub fn update() -> Result<(), PeachConfigError> {
//    // update peach-config (update itself) then run update on all other microservices
//    args = parser.parse_args()
//
//    // if -list then just show updates available without running them
//    if args.list:
//        list_available_updates()
//    // just update self
//    elif args.self:
//        run_update_self()
//    // just update other microservices
//    elif args.microservices:
//        update_microservices(purge=args.purge)
//    // update self and then update other microservices
//    else:
//        run_update_self()
//        subprocess.check_call(['/usr/bin/peach-config', 'update', '--microservices'])
//}

/// checks if there are any PeachCloud updates available and displays them
pub fn list_available_updates() -> Result<(), PeachConfigError> {
    cmd(&["apt-get", "update"])?;
    let output = get_output(&["apt", "list", "--upgradable"])?;
    let lines = output.split("\n");
    // filter down to just lines which are one of the services
    let upgradeable = lines
        .into_iter()
        .filter(|x| SERVICES.iter().any(|s| x == s));
    info!("upgradeable: {:?}", upgradeable);
    // TODO: format as json
    Ok(())
}

//
//def init_update_parser(parser):
//    // update argument parser
//    parser.add_argument("-m", "--microservices", help="update all other peach microservices", action="store_true")
//    parser.add_argument("-l", "--list", help="list if there are any updates available without running them", action="store_true")
//    parser.add_argument("-s", "--self", help="update peach-config", action="store_true")
//    parser.add_argument("-p", "--purge", help="purge old installations when updating", action="store_true")
//    return parser
//
