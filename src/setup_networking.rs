use log::info;
use std::path::Path;

use crate::error::PeachConfigError;
use crate::utils::{cmd, conf};


/// Idempotent script to configure a Debian installation to use
/// systemd-networkd for general networking. The script configures the eth0,
/// wlan0 and ap0 interfaces. This configuration allows switching between
/// wireless client mode (wlan0) and wireless access point mode (ap0)
pub fn configure_networking() -> Result<(), PeachConfigError> {
    info!("[ INSTALLING SYSTEM REQUIREMENTS ]");
    cmd(&["apt", "install", "-y", "libnss-resolve"])?;

    info!("[ SETTING HOST ]");
    cmd(&["cp", &conf("hostname"), "/etc/hostname"])?;
    cmd(&["cp", &conf("hosts"), "/etc/hosts"])?;

    info!("[ DEINSTALLING CLASSIC NETWORKING ]");
    cmd(&[
        "apt-get",
        "autoremove",
        "-y",
        "ifupdown",
        "dhcpcd5",
        "isc-dhcp-client",
        "isc-dhcp-common",
        "rsyslog",
    ])?;
    cmd(&[
        "apt-mark",
        "hold",
        "ifupdown",
        "dhcpcd5",
        "isc-dhcp-client",
        "isc-dhcp-common",
        "rsyslog",
        "openresolv",
    ])?;
    cmd(&["rm", "-rf", "/etc/network", "/etc/dhcp"])?;

    info!("[ SETTING UP SYSTEMD-RESOLVED & SYSTEMD-NETWORKD ]");
    cmd(&["apt-get", "autoremove", "-y", "avahi-daemon"])?;
    cmd(&["apt-mark", "hold", "avahi-daemon", "libnss-mdns"])?;
    cmd(&[
        "ln",
        "-sf",
        "/run/systemd/resolve/stub-resolv.conf",
        "/etc/resolv.conf",
    ])?;
    cmd(&[
        "systemctl",
        "enable",
        "systemd-networkd.service",
        "systemd-resolved.service",
    ])?;

    info!("[ CREATING INTERFACE FILE FOR WIRED CONNECTION ]");
    cmd(&[
        "cp",
        &conf("network/04-wired.network"),
        "/etc/systemd/network/04-wired.network",
    ])?;

    info!("[ SETTING UP WPA_SUPPLICANT AS WIFI CLIENT WITH WLAN0 ]");
    // to avoid overwriting previous credentials, only copy file if it doesn't already exist
    let wlan0 = "/etc/wpa_supplicant/wpa_supplicant-wlan0.conf";
    if !Path::new(wlan0).exists() {
        cmd(&["cp", &conf("network/wpa_supplicant-wlan0.conf"), wlan0])?;
        cmd(&["chmod", "660", wlan0])?;
        cmd(&["chown", "root:netdev", wlan0])?;
    }
    cmd(&["systemctl", "disable", "wpa_supplicant.service"])?;
    cmd(&["systemctl", "enable", "wpa_supplicant@wlan0.service"])?;

    info!("[ CREATING BOOT SCRIPT TO COPY NETWORK CONFIGS ]");
    cmd(&[
        "cp",
        &conf("network/copy-wlan.sh"),
        "/usr/local/bin/copy-wlan.sh",
    ])?;
    cmd(&["chmod", "770", "/usr/local/bin/copy-wlan.sh"])?;
    cmd(&[
        "cp",
        &conf("network/copy-wlan.service"),
        "/etc/systemd/system/copy-wlan.service",
    ])?;
    cmd(&["systemctl", "enable", "copy-wlan.service"])?;

    info!("[ SETTING UP WPA_SUPPLICANT AS ACCESS POINT WITH AP0 ]");
    cmd(&[
        "cp",
        &conf("network/wpa_supplicant-ap0.conf"),
        "/etc/wpa_supplicant/wpa_supplicant-ap0.conf",
    ])?;
    cmd(&[
        "chmod",
        "600",
        "/etc/wpa_supplicant/wpa_supplicant-ap0.conf",
    ])?;

    info!("[ CONFIGURING INTERFACES ]");
    cmd(&[
        "cp",
        &conf("network/08-wlan0.network"),
        "/etc/systemd/network/08-wlan0.network",
    ])?;
    cmd(&[
        "cp",
        &conf("network/12-ap0.network"),
        "/etc/systemd/network/12-ap0.network",
    ])?;

    info!("[ MODIFYING SERVICE FOR ACCESS POINT TO USE AP0 ]");
    cmd(&["systemctl", "disable", "wpa_supplicant@ap0.service"])?;
    cmd(&[
        "cp",
        &conf("network/wpa_supplicant@ap0.service"),
        "/etc/systemd/system/wpa_supplicant@ap0.service",
    ])?;

    info!("[ SETTING WLAN0 TO RUN AS CLIENT ON STARTUP ]");
    cmd(&["systemctl", "enable", "wpa_supplicant@wlan0.service"])?;
    cmd(&["systemctl", "disable", "wpa_supplicant@ap0.service"])?;

    info!("[ CREATING ACCESS POINT AUTO-DEPLOY SCRIPT ]");
    cmd(&[
        "cp",
        &conf("ap_auto_deploy.sh"),
        "/usr/local/bin/ap_auto_deploy",
    ])?;

    info!("[ CONFIGURING ACCESS POINT AUTO-DEPLOY SERVICE ]");
    cmd(&[
        "cp",
        &conf("network/ap-auto-deploy.service"),
        "/etc/systemd/system/ap-auto-deploy.service",
    ])?;
    cmd(&[
        "cp",
        &conf("network/ap-auto-deploy.timer"),
        "/etc/systemd/system/ap-auto-deploy.timer",
    ])?;

    info!("[ NETWORKING HAS BEEN CONFIGURED ]");
    Ok(())
}
