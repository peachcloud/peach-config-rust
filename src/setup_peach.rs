use log::info;
use snafu::ResultExt;
use std::fs;

use crate::error::{FileWriteError, PeachConfigError};
use crate::setup_networking::configure_networking;
use crate::setup_peach_deb::setup_peach_deb;
use crate::generate_manifest::save_hardware_config;
use crate::update::update_microservices;
use crate::utils::{cmd, conf, create_group_if_doesnt_exist, does_user_exist, get_output};
use crate::RtcOption;


/// Idempotent setup of PeachCloud device which sets up networking configuration,
/// configures the peachcloud apt repository, installs system dependencies,
/// installs microservices, and creates necessary system groups and users.
///
/// # Arguments
///
/// * `no_input` - a bool, if true, runs the script without requiring user interaction
/// * `default_locale` - a bool, if true, sets the default locale of the device to en_US.UTF-8
/// * `i2c` - a bool, if true, setup i2c configurations for peach-menu
/// * `rtc` - an optional enum, which if provided indicates the model number of the real-time
///           clock being used
///
/// If any command in the script returns an error (non-zero exit status) a PeachConfigError
/// is returned, otherwise an Ok is returned.
pub fn setup_peach(
    no_input: bool,
    default_locale: bool,
    i2c: bool,
    rtc: Option<RtcOption>,
) -> Result<(), PeachConfigError> {
    info!("[ RUNNING SETUP PEACH ]");

    // list of system users for (micro)services
    let users = [
        "peach-buttons",
        "peach-menu",
        "peach-monitor",
        "peach-network",
        "peach-oled",
        "peach-stats",
        "peach-web",
    ];

    // Update Pi and install requirements
    info!("[ UPDATING OPERATING SYSTEM ]");
    //    cmd(&["apt-get", "update", "-y"])?;
    //    cmd(&["apt-get", "upgrade", "-y"])?;

    info!("[ INSTALLING SYSTEM REQUIREMENTS ]");
    cmd(&[
        "apt-get",
        "install",
        "vim",
        "man-db",
        "locales",
        "iw",
        "git",
        "python-smbus",
        "i2c-tools",
        "build-essential",
        "curl",
        "libnss-resolve",
        "mosh",
        "sudo",
        "pkg-config",
        "libssl-dev",
        "nginx",
        "wget",
        "-y",
    ])?;

    // Create system groups first
    info!("[ CREATING SYSTEM GROUPS ]");
    create_group_if_doesnt_exist("peach")?;
    create_group_if_doesnt_exist("gpio-user")?;

    //  Add the system users
    info!("[ ADDING SYSTEM USER ]");
    if no_input {
        //  if no input, then peach user starts with password peachcloud
        let default_password = "peachcloud";
        let enc_password = get_output(&["openssl", "passwd", "-crypt", default_password])?;
        info!("[ CREATING SYSTEM USER WITH DEFAULT PASSWORD ]");
        if !(does_user_exist("peach")?) {
            cmd(&[
                "/usr/sbin/useradd",
                "-m",
                "-p",
                &enc_password,
                "-g",
                "peach",
                "-s",
                "/bin/bash",
                "peach",
            ])?;
        }
    } else {
        cmd(&["/usr/sbin/adduser", "peach"])?;
    }
    cmd(&["usermod", "-aG", "sudo", "peach"])?;
    cmd(&["usermod", "-aG", "peach", "peach"])?;

    info!("[ CREATING SYSTEM USERS ]");
    //  Peachcloud microservice users
    for user in users {
        //  Create new system user without home directory and add to `peach` group
        cmd(&[
            "/usr/sbin/adduser",
            "--system",
            "--no-create-home",
            "--ingroup",
            "peach",
            user,
        ])?;
    }

    info!("[ ASSIGNING GROUP MEMBERSHIP ]");
    cmd(&[
        "/usr/sbin/usermod",
        "-a",
        "-G",
        "gpio-user",
        "peach-buttons",
    ])?;
    cmd(&["/usr/sbin/usermod", "-a", "-G", "netdev", "peach-network"])?;
    cmd(&["/usr/sbin/usermod", "-a", "-G", "i2c", "peach-oled"])?;

    //  Overwrite configuration files
    info!("[ CONFIGURING OPERATING SYSTEM ]");
    info!("[ CONFIGURING GPIO ]");
    cmd(&[
        "cp",
        &conf("50-gpio.rules"),
        "/etc/udev/rules.d/50-gpio.rules",
    ])?;

    if i2c {
        info!("[ CONFIGURING I2C ]");
        cmd(&["mkdir", "-p", "/boot/firmware/overlays"])?;
        cmd(&[
            "cp",
            &conf("mygpio.dtbo"),
            "/boot/firmware/overlays/mygpio.dtbo",
        ])?;
        cmd(&["cp", &conf("config.txt_i2c"), "/boot/firmware/config.txt"])?;
        cmd(&["cp", &conf("modules"), "/etc/modules"])?;
    }

    if let Some(rtc_model) = &rtc {
        if i2c {
            match rtc_model {
                RtcOption::DS1307 => {
                    info!("[ CONFIGURING DS1307 RTC MODULE ]");
                    cmd(&[
                        "cp",
                        &conf("config.txt_ds1307"),
                        "/boot/firmware/config.txt",
                    ])?;
                }
                RtcOption::DS3231 => {
                    info!("[ CONFIGURING DS3231 RTC MODULE ]");
                    cmd(&[
                        "cp",
                        &conf("config.txt_ds3231"),
                        "/boot/firmware/config.txt",
                    ])?;
                }
            }
            cmd(&["cp", &conf("modules_rtc"), "/etc/modules"])?;
            cmd(&[
                "cp",
                &conf("activate_rtc.sh"),
                "/usr/local/bin/activate_rtc",
            ])?;
            cmd(&[
                "cp",
                &conf("activate-rtc.service"),
                "/etc/systemd/system/activate-rtc.service",
            ])?;
            cmd(&["systemctl", "daemon-reload"])?;
            cmd(&["systemctl", "enable", "activate-rtc"])?;
        }
    }

    info!("[ CONFIGURING NGINX ]");
    cmd(&[
        "cp",
        &conf("peach.conf"),
        "/etc/nginx/sites-available/peach.conf",
    ])?;
    cmd(&[
        "ln",
        "-sf",
        "/etc/nginx/sites-available/peach.conf",
        "/etc/nginx/sites-enabled/",
    ])?;

    if !no_input {
        info!("[ CONFIGURING LOCALE ]");
        cmd(&["dpkg-reconfigure", "locales"])?;
        //  this is specified as an argument, so a user can run this script in no-input  mode without updating their locale
        //  if they have already set it
        if default_locale {
            info!("[ SETTING DEFAULT LOCALE TO en_US.UTF-8 FOR COMPATIBILITY  ]");
            cmd(&[
                "sed",
                "-i",
                "-e",
                "s///  en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/",
                "/etc/locale.gen",
            ])?;
            fs::write("/etc/default/locale", "LANG=\"en_US.UTF-8\"").context(FileWriteError {
                file: "/etc/default/locale".to_string(),
            })?;
            cmd(&["dpkg-reconfigure", "--frontend=noninteractive", "locales"])?;
        }
    }

    info!("[ CONFIGURING CONSOLE LOG-LEVEL PRINTING ]");
    // TODO: for now commenting this out, because its throwing an error
//    cmd(&["sysctl", "-w", "kernel.printk=4 4 1 7"])?;

    info!("[ CONFIGURING SUDOERS ]");
    cmd(&["mkdir", "-p", "/etc/sudoers.d"])?;
    cmd(&["cp", &conf("shutdown"), "/etc/sudoers.d/shutdown"])?;

    info!("[ CONFIGURING PEACH APT REPO ]");
    setup_peach_deb()?;

    info!("[ INSTALLING PEACH MICROSERVICES ]");
    update_microservices()?;

    info!("[ CONFIGURING NETWORKING ]");
    configure_networking()?;

    info!("[ SAVING LOG OF HARDWARE CONFIGURATIONS ]");
    save_hardware_config(i2c, rtc);

    info!("[ PEACHCLOUD SETUP COMPLETE ]");
    info!("[ ------------------------- ]");
    info!("[ please reboot your device ]");
    Ok(())
}
