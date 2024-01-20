#!/bin/sh
cargo b --release
sudo setcap CAP_NET_ADMIN=eip target/release/tcp_rust

# Check if tun0 exists, and if so, bring it down
if ip link show tun0 >/dev/null 2>&1; then
  sudo ip link set down dev tun0
  sudo ip tuntap del tun0 mode tun
fi

# Create and configure tun0
sudo ip tuntap add tun0 mode tun
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0

# Run the binary in the background and capture its PID
target/release/tcp_rust &
PID=$!

# Wait for a moment to ensure the process has started
sleep 1

# Wait for the background process to complete
wait $PID
