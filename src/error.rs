#![allow(clippy::nonstandard_macro_braces)]
pub use snafu::ResultExt;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[allow(clippy::enum_variant_names)]
#[snafu(visibility(pub(crate)))]
pub enum PeachConfigError {
    #[snafu(display("Command not found: \"{}\"", command))]
    CmdIoError {
        source: std::io::Error,
        command: String,
    },
    #[snafu(display("\"{}\" returned an error. {}", command, msg))]
    CmdError { msg: String, command: String },
    #[snafu(display("Command could not parse stdout: \"{}\"", command))]
    CmdParseOutputError {
        source: std::str::Utf8Error,
        command: String,
    },
    #[snafu(display("Failed to write file: {}", file))]
    FileWriteError {
        file: String,
        source: std::io::Error,
    },
    #[snafu(display("Failed to read file: {}", file))]
    FileReadError {
        file: String,
        source: std::io::Error,
    },
    #[snafu(display("Error serializing json: {}", source))]
    SerdeError { source: serde_json::Error },
}

impl From<std::io::Error> for PeachConfigError {
    fn from(err: std::io::Error) -> PeachConfigError {
        PeachConfigError::CmdIoError {
            source: err,
            command: "unknown".to_string(),
        }
    }
}

impl From<serde_json::Error> for PeachConfigError {
    fn from(err: serde_json::Error) -> PeachConfigError {
        PeachConfigError::SerdeError { source: err }
    }
}
