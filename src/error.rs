pub use snafu::ResultExt;
use snafu::Snafu;
use std::error;
pub type BoxError = Box<dyn error::Error>;

#[derive(Debug, Snafu)]
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
}

impl From<std::io::Error> for PeachConfigError {
    fn from(err: std::io::Error) -> PeachConfigError {
        PeachConfigError::CmdIoError {
            source: err,
            command: "unknown".to_string(),
        }
    }
}
