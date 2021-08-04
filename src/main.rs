mod constants;
mod error;
mod generate_manifest;
mod setup_networking;
mod setup_peach;
mod setup_peach_deb;
mod update;
mod utils;

use clap::arg_enum;
use log::error;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::generate_manifest::generate_manifest;
use crate::setup_peach::setup_peach;
use crate::update::update;

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
    /// Prints json manifest of peach configurations
    #[structopt(name = "manifest")]
    Manifest,

    /// Idempotent setup of PeachCloud
    #[structopt(name = "setup")]
    Setup(SetupOpts),

    /// Updates all PeachCloud microservices
    #[structopt(name = "update")]
    Update(UpdateOpts),
}

#[derive(StructOpt, Debug)]
struct SetupOpts {
    /// Setup i2c configurations
    #[structopt(short, long)]
    i2c: bool,
    /// Optionally select which model of real-time-clock is being used,
    /// {ds1307, ds3231}
    #[structopt(short, long)]
    rtc: Option<RtcOption>,
    /// Run peach-config in non-interactive mode
    #[structopt(short, long)]
    no_input: bool,
    /// Use the default en_US.UTF-8 locale for compatability
    #[structopt(short, long)]
    default_locale: bool,
}

#[derive(StructOpt, Debug)]
pub struct UpdateOpts {
    /// Only update other microservices and not peach-config
    #[structopt(short, long)]
    microservices: bool,
    /// Only update peach-config and not other microservices
    #[structopt(short, long = "--self")]
    self_only: bool,
    /// List microservices which are available for updating
    #[structopt(short, long)]
    list: bool,
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

    // switch based on subcommand
    if let Some(subcommand) = opt.commands {
        match subcommand {
            PeachConfig::Setup(cfg) => {
                match setup_peach(cfg.no_input, cfg.default_locale, cfg.i2c, cfg.rtc) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("peach-config encountered an error: {}", err)
                    }
                }
            }
            PeachConfig::Manifest => match generate_manifest() {
                Ok(_) => {}
                Err(err) => {
                    error!(
                        "peach-config countered an error generating manifest: {}",
                        err
                    )
                }
            },
            PeachConfig::Update(opts) => match update(opts) {
                Ok(_) => {}
                Err(err) => {
                    error!("peach-config encountered an error during update: {}", err)
                }
            },
        }
    }
}
