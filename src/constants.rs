// directory on peachcloud device where CONF files are store
// before they are copied to their eventual locations
pub const CONF: &str = "/var/lib/peachcloud/conf";

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
