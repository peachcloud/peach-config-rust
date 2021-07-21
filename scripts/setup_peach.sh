#!/usr/bin/env bash
set -e

function usage() {
  echo "
    Usage :
      `basename $0` [-a] [-d <DISTRIB>] [-h]
    Options :
      -a      Enable automatic mode. No questions are asked.
              This does not perform the post-install step.
      -d      Choose the distribution to install ('stable', 'testing', 'unstable').
              Defaults to 'stable'
      -f      Ignore checks before starting the installation. Use only if you know
              what you are doing.
      -h      Prints this help and exit
    "
}

function parse_options()
{
    I2C=0
    RTC=""
    NOINPUT=0

    while getopts ":aid:fh" option; do
        case $option in
            n)
                NOINPUT=1
                ;;
            r)
                RTC=$OPTARG
                ;;
            i)
                I2C=1
                ;;
            h)
                usage
                exit 0
                ;;
            :)
                usage
                exit 1
                ;;
            \?)
                usage
                exit 1
                ;;
        esac
    done
}

# Create list of system users for (micro)services
users=("peach-buttons" "peach-menu" "peach-monitor" "peach-network" "peach-oled" "peach-stats" "peach-web")

# Update Pi and install requirements
echo "[ UPDATING OPERATING SYSTEM ]"
subprocess.call(["apt-get", "update", "-y"])
subprocess.call(["apt-get", "upgrade", "-y"])

echo "[ INSTALLING SYSTEM REQUIREMENTS ]"
subprocess.call(["apt-get",
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
                 "-y"])

# Create system groups first
echo "[ CREATING SYSTEM GROUPS ]"
subprocess.call(["/usr/sbin/groupadd", "peach"])
subprocess.call(["/usr/sbin/groupadd", "gpio-user"])

# Add the system users
echo "[ ADDING SYSTEM USER ]"
if no_input:
    # if no input, then peach user starts with password peachcloud
    default_password = "peachcloud"
    enc_password = crypt.crypt(default_password, "22")
    echo "[ CREATING SYSTEM USER WITH DEFAULT PASSWORD ]"
    subprocess.call(["/usr/sbin/useradd", "-m", "-p", enc_password, "-g", "peach", "-s", "/bin/bash", "peach"])
else:
    subprocess.call(["/usr/sbin/adduser", "peach"])
subprocess.call(["usermod", "-aG", "sudo", "peach"])
subprocess.call(["usermod", "-aG", "peach", "peach"])

echo "[ CREATING SYSTEM USERS ]"
# Peachcloud microservice users
for user in users:
    # Create new system user without home directory and add to `peach` group
    subprocess.call(["/usr/sbin/adduser", "--system",
                     "--no-create-home", "--ingroup", "peach", user])

echo "[ ASSIGNING GROUP MEMBERSHIP ]"
subprocess.call(["/usr/sbin/usermod", "-a", "-G",
                 "gpio-user", "peach-buttons"])
subprocess.call(["/usr/sbin/usermod", "-a", "-G", "netdev", "peach-network"])
subprocess.call(["/usr/sbin/usermod", "-a", "-G", "i2c", "peach-oled"])

# Overwrite configuration files
echo "[ CONFIGURING OPERATING SYSTEM ]"
echo "[ CONFIGURING GPIO ]"
subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/50-gpio.rules"),
                 "/etc/udev/rules.d/50-gpio.rules"])

if i2c:
    echo "[ CONFIGURING I2C ]"
    if not os.path.exists("/boot/firmware/overlays"):
        os.mkdir("/boot/firmware/overlays")
    subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/mygpio.dtbo"),
                     "/boot/firmware/overlays/mygpio.dtbo"])
    subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/config.txt_i2c"), "/boot/firmware/config.txt"])
    subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/modules"), "/etc/modules"])

if rtc and i2c:
    if rtc == "ds1307":
        echo "[ CONFIGURING DS1307 RTC MODULE ]"
        subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/config.txt_ds1307"),
                         "/boot/firmware/config.txt"])
    elif rtc == "ds3231":
        echo "[ CONFIGURING DS3231 RTC MODULE ]"
        subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/config.txt_ds3231"),
                         "/boot/firmware/config.txt"])
    subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/modules_rtc"), "/etc/modules"])
    subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/activate_rtc.sh"),
                     "/usr/local/bin/activate_rtc"])
    subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/activate-rtc.service"),
                     "/etc/systemd/system/activate-rtc.service"])
    subprocess.call(["systemctl", "daemon-reload"])
    subprocess.call(["systemctl", "enable", "activate-rtc"])

echo "[ CONFIGURING NGINX ]"
subprocess.call(
    ["cp", os.path.join(PROJECT_PATH, "conf/peach.conf"), "/etc/nginx/sites-available/peach.conf"])
subprocess.call(["ln",
                 "-sf",
                 "/etc/nginx/sites-available/peach.conf",
                 "/etc/nginx/sites-enabled/"])

if not no_input:
    echo "[ CONFIGURING LOCALE ]"
    subprocess.call(["dpkg-reconfigure", "locales"])

# this is specified as an argument, so a user can run this script in no-input  mode without updating their locale
# if they have already set it
if default_locale:
    echo "[ SETTING DEFAULT LOCALE TO en_US.UTF-8 FOR COMPATIBILITY  ]"
    subprocess.call(["sed", "-i", "-e","s/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/", "/etc/locale.gen"])
    with open('/etc/default/locale', 'w') as f:
        print('LANG="en_US.UTF-8"', file=f)
    subprocess.call(["dpkg-reconfigure", "--frontend=noninteractive", "locales"])

echo "[ CONFIGURING CONSOLE LOG-LEVEL PRINTING ]"
subprocess.call(["sysctl", "-w", "kernel.printk=4 4 1 7"])

echo "[ CONFIGURING SUDOERS ]"
if not os.path.exists("/etc/sudoers.d"):
    os.mkdir("/etc/sudoers.d")
subprocess.call(["cp", os.path.join(PROJECT_PATH, "conf/shutdown"), "/etc/sudoers.d/shutdown"])