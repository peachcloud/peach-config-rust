mod constants;
mod error;
mod setup_networking;
mod setup_peach;
mod setup_peach_deb;
mod update;
mod utils;
mod generate_manifest;

use serde::{Deserialize, Serialize};
use clap::arg_enum;
use log::{error, info};
use structopt::StructOpt;

use crate::setup_peach::setup_peach;
use crate::update::update_microservices;
use crate::generate_manifest::generate_manifest;


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

    #[structopt(name = "update")]
    Update,
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
    /// enum options for real-time clock choices
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    #[allow(clippy::enum_variant_names)]
    #[derive(Serialize, Deserialize)]
    pub enum RtcOption {
        DS1307,
        DS3231
    }
}

fn main() {
    // initialize the logger
    env_logger::init();

    // parse cli arguments
    let opt = Opt::from_args();

    info!("++ running peach-config");
    if let Some(subcommand) = opt.commands {
        match subcommand {
            PeachConfig::Setup(cfg) => {
                match setup_peach(cfg.no_input, cfg.default_locale, cfg.i2c, cfg.rtc) {
                    Ok(_) => {},
                    Err(err) => {
                        error!("peach-config encounter an error: {}", err)
                    }
                }
            }
            PeachConfig::Manifest => {
                match generate_manifest() {
                    Ok(_) => {},
                    Err(err) => {
                        error!("encounter an error generating manifest: {}", err)
                    }
                }
            },
            PeachConfig::Update => {
                match update_microservices() {
                    Ok(_) => {},
                    Err(err) => {
                        error!("encounter an error during update: {}", err)
                    }
                }
            }
        }
    }
}
