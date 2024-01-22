#!/bin/sh

# Build the binary
cargo b --release
sudo setcap CAP_NET_ADMIN=eip target/release/tcp_rust

# Function to clean up tun0
cleanup_tun0() {
  if ip link show tun0 >/dev/null 2>&1; then
    echo "Bringing down tun0..."
    sudo ip link set down dev tun0
    echo "Deleting tun0..."
    sudo ip tuntap del tun0 mode tun
  fi
}

# Function to handle signals
handle_signal() {
  echo "Received signal, cleaning up and terminating..."
  cleanup_tun0
  sudo kill $PID
  exit 1
}

# Trap signals to call the handle_signal function
trap handle_signal INT TERM

# Check if tun0 exists, and if so, bring it down
if ip link show tun0 >/dev/null 2>&1; then
  cleanup_tun0
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

# Clean up tun0 before exiting
cleanup_tun0
