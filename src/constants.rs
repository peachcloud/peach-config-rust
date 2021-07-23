// directory on peachcloud device where CONF files are store
// before they are copied to their eventual locations
pub const CONF: &str = "/var/lib/peachcloud/conf";

// list of package names which are installed via apt-get
pub const SERVICES: [&str; 11] = [
    "peach-oled",
    "peach-network",
    "peach-stats",
    "peach-web",
    "peach-menu",
    "peach-buttons",
    "peach-monitor",
    "peach-probe",
    "peach-dyndns-updater",
    "peach-go-sbot",
    "peach-config",
];

// file path to where current hardware configurations are stored
// note: this is stored separately from /var/lib/peachcloud/config.yml
// because it is not a configuration which should be manually edited
// the values in the hardware_config.json are a log of what peach-config configured
// whereas the values in config.yml can be manually modified if needed
pub const HARDWARE_CONFIG_FILE: &str = "/var/lib/peachcloud/hardware_config.json";