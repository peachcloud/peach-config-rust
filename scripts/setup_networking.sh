#!/usr/bin/env bash
set -e

# conf dir where files are copied from
CONF=/var/lib/conf

echo "[ INSTALLING SYSTEM REQUIREMENTS ]"
apt install -y libnss-resolve

echo "[ SETTING HOST ]"
cp $CONF/hostname etc/hostname
cp $CONF/hosts /etc/hosts

echo "[ DEINSTALLING CLASSIC NETWORKING ]"
apt-get autoremove -y ifupdown dhcpcd5 isc-dhcp-client isc-dhcp-common rsyslog
apt-mark hold ifupdown dhcpcd5 isc-dhcp-client isc-dhcp-common rsyslog openresolv
rm -rf /etc/network /etc/dhcp

echo "[ SETTING UP SYSTEMD-RESOLVED & SYSTEMD-NETWORKD ]"
apt-get autoremove -y avahi-daemon
apt-mark hold avahi-daemon libnss-mdns
ln -sf /run/systemd/resolve/stub-resolv.conf /etc/resolv.conf
systemctl enable systemd-networkd.service systemd-resolved.service

echo "[ CREATING INTERFACE FILE FOR WIRED CONNECTION ]"
cp $CONF/network/04-wired.network /etc/systemd/network/04-wired.network

echo "[ SETTING UP WPA_SUPPLICANT AS WIFI CLIENT WITH WLAN0 ]"
# to avoid overwriting previous wifi credentials, only copy file if it doesn't already exist
WLAN0_CONF=/etc/wpa_supplicant/wpa_supplicant-wlan0.conf
if test -f "$WLAN0_CONF"; then
    cp $CONF/network/wpa_supplicant-wlan0.conf $WLAN0_CONF
fi
chmod 660 $WLAN0_CONF
chown root:netdev $WLAN0_CONF
systemctl disable wpa_supplicant.service
systemctl enable wpa_supplicant@wlan0.service

echo "[ CREATING BOOT SCRIPT TO COPY NETWORK CONFIGS ]"
cp $CONF/network/copy-wlan.sh /usr/local/bin/copy-wlan.sh
chmod 770 /usr/local/bin/copy-wlan.sh
cp $CONF/network/copy-wlan.service /etc/systemd/system/copy-wlan.service
systemctl enable copy-wlan.service

echo "[ SETTING UP WPA_SUPPLICANT AS ACCESS POINT WITH AP0 ]"
cp $CONF/network/wpa_supplicant-ap0.conf /etc/wpa_supplicant/wpa_supplicant-ap0.conf
chmod 600 /etc/wpa_supplicant/wpa_supplicant-ap0.conf

echo "[ CONFIGURING INTERFACES ]"
cp $CONF/network/08-wlan0.network /etc/systemd/network/08-wlan0.network
cp $CONF/network/12-ap0.network /etc/systemd/network/12-ap0.network

echo "[ MODIFYING SERVICE FOR ACCESS POINT TO USE AP0 ]"
systemctl disable wpa_supplicant@ap0.service
cp $CONF/network/wpa_supplicant@ap0.service /etc/systemd/system/wpa_supplicant@ap0.service

echo "[ SETTING WLAN0 TO RUN AS CLIENT ON STARTUP ]"
systemctl enable wpa_supplicant@wlan0.service
systemctl disable wpa_supplicant@ap0.service

echo "[ CREATING ACCESS POINT AUTO-DEPLOY SCRIPT ]"
cp $CONF/ap_auto_deploy.sh /usr/local/bin/ap_auto_deploy

echo "[ CONFIGURING ACCESS POINT AUTO-DEPLOY SERVICE ]"
cp $CONF/network/ap-auto-deploy.service /etc/systemd/system/ap-auto-deploy.service
cp $CONF/network/ap-auto-deploy.timer /etc/systemd/system/ap-auto-deploy.timer

echo "[ NETWORKING HAS BEEN CONFIGURED ]"
