mod constants;
mod error;
mod setup_networking;
mod setup_peach;
mod setup_peach_deb;
mod update;
mod utils;

use clap::arg_enum;
use log::{error, info};
use structopt::StructOpt;

use crate::setup_peach::setup_peach;
use crate::update::update_microservices;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "peach-config",
    about = "a CLI tool for updating, installing and configuring PeachCloud"
)]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Option<PeachConfig>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "peach-config", about = "about")]
enum PeachConfig {
    #[structopt(name = "manifest")]
    Manifest,

    #[structopt(name = "setup")]
    Setup(SetupOpts),
}

#[derive(StructOpt, Debug)]
struct SetupOpts {
    #[structopt(short, long)]
    i2c: bool,
    #[structopt(short, long)]
    rtc: Option<RtcOption>,
    #[structopt(short, long)]
    no_input: bool,
    #[structopt(short, long)]
    default_locale: bool,
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    #[allow(clippy::enum_variant_names)]
    pub enum RtcOption {
        DS1307,
        DS3231
    }
}

//// enum options for real-time clock choices
//#[derive(PartialEq, Debug)]
//pub enum RtcOption {
//    DS1307,
//    DS3231
//}

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
    if let Some(subcommand) = opt.commands {
        match subcommand {
            PeachConfig::Setup(cfg) => {
                match setup_peach(cfg.no_input, cfg.default_locale, cfg.i2c, cfg.rtc) {
                    Ok(_) => {
                        info!("++ succesfully configured peach")
                    }
                    Err(err) => {
                        error!("peach-config encounter an error: {}", err)
                    }
                }
            }
            PeachConfig::Manifest => {
                println!("++ generating manifest");
            }
        }
    }
}
