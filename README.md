# peach-config

[![Build Status](https://travis-ci.com/peachcloud/peach-config.svg?branch=main)](https://travis-ci.com/peachcloud/peach-config) 
![Generic badge](https://img.shields.io/badge/version-0.1.9-<COLOR>.svg)

Rust crate which provides a CLI tool for installing and updating PeachCloud. 



## Installation From PeachCloud Disc Image

The recommended way to install PeachCloud is to download the latest PeachCloud disc image from http://releases.peachcloud.org, 
and flash it to an SD card. peach-config is included as part of this disc image, and can then 
be used as a tool for updating PeachCloud as needed. 

You can find detailed instructions on setting up PeachCloud from a PeachCloud disc image [here](docs/installation-from-peach-disc-image.md). 


## Installation From Debian Disc Image

You can find a guide for installing plain Debian onto a Raspberry pi [here](docs/installation-from-debian-disc-image.md).

Once you have Debian running on your pi, you can install peach-config by adding the PeachCloud apt repository and using apt. 

To add the PeachCloud Debian package archive as an apt source, run the following commands from your Pi:

``` bash
echo "deb http://apt.peachcloud.org/ buster main" > /etc/apt/sources.list.d/peach.list
wget -O - http://apt.peachcloud.org/pubkey.gpg | sudo apt-key add -
```

You can then install peach-config with apt:

``` bash
sudo apt-get update
sudo apt-get install python3-peach-config
```

Alternatively you can run the following one-liner, which does all of the above:
> curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/peachcloud/peach-config-rust/main/install.sh | sh

peach-config has only been tested on a Raspberry Pi 3 B+ running Debian 10. 


## Usage

The peach-config debian module installs a command-line tool to `/usr/bin/peach-config`.

`peach-config` is a tool for installing PeachCloud and for updating it. 

`peach-config -h` shows the help menu:

```bash
USAGE:
    peach-config [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    manifest    Prints json manifest of peach configurations
    setup       Idempotent setup of PeachCloud
    update      Updates all PeachCloud microservices
```

The setup command takes a few different parameters to customize configuration. 
```bash
USAGE:
    peach-config setup [FLAGS] [OPTIONS]

FLAGS:
    -d, --default-locale    Use the default en_US.UTF-8 locale for compatability
    -h, --help              Prints help information
    -i, --i2c               Setup i2c configurations
    -n, --no-input          Run peach-config in non-interactive mode
    -V, --version           Prints version information

OPTIONS:
    -r, --rtc <rtc>    Optionally select which model of real-time-clock is being used {ds1307, ds3231}
```

I2C configuration is necessary for the OLED display and physical interface to work correctly. RTC configuration is required for the real-time clock to work correctly. When passing the `-r` flag, the type of real-time clock module must be included (either ds1307 or ds3231). Selecting real-time clock configuration will not work if the I2C flag is not selected (in other words, the real-time clock requires I2C).

Run the script as follows for a full installation and configuration with I2C and the ds3231 RTC module:

`peach-config setup -i -r ds3231 -n -d`


## Licensing

AGPL-3.0