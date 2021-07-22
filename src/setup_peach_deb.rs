use crate::error::PeachConfigError;
use crate::utils::{cmd, conf};


/// Adds apt.peachcloud.org to the list of debian apt sources and sets the public key appropriately
pub fn setup_peach_deb() -> Result<(), PeachConfigError> {
    cmd(&["cp", &conf("peach.list"), "/etc/apt/sources.list.d/peach.list"])?;
    cmd(&["wget", "-O", "/tmp/pubkey.gpg", "http://apt.peachcloud.org/pubkey.gpg"])?;
    cmd(&["apt-key", "add", "/tmp/pubkey.gpg"])?;
    cmd(&["rm", "/tmp/pubkey.gpg"])?;
    Ok(())
}


