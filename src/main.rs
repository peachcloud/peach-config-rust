mod error;
mod setup_peach;
mod utils;
mod constants;

use log::{info, error};
use structopt::StructOpt;

use crate::setup_peach::setup_peach;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "peach-config",
    about = "a CLI tool for updating, installing and configuring PeachCloud"
)]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,
}


// enum options for real-time clock choices
#[derive(PartialEq)]
pub enum RtcOption {
   DS1307,
   DS3231,
   None,
}

fn main() {
    // initialize the logger
    env_logger::init();

    // parse cli arguments
    let opt = Opt::from_args();

    // debugging what was parsed
    if opt.verbose {
        info!("using verbose mode")
    }

    info!("++ running peach-config");
    match setup_peach(true, true, true, RtcOption::None) {
        Ok(_) => {
            info!("++ succesfully configured peach")
        },
        Err(err) => {
            error!("peach-config encounter an error: {}", err)
        }
    }
}
