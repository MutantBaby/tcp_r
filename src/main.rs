use std::io::Result;

use tun_tap::{Iface, Mode};

extern crate tun_tap;

fn main() -> Result<()> {
    let iface0: Iface = Iface::new("tun0", Mode::Tun)?;
    let mut buff: Vec<u8> = vec![0; 1504];

    loop {
        let n_bytes: usize = iface0.recv(&mut buff)?;
        let flags: u16 = u16::from_be_bytes([buff[0], buff[1]]);
        let proto: u16 = u16::from_be_bytes([buff[2], buff[3]]);

        if proto != 0x0800 {
            // no IPv4
            println!("Not an IPv4 packet, skipping");
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buff[4..n_bytes]) {
            Ok(packet) => {
                println!(
                    "Received {} bytes (Flags {:x}, \tProto {:x},) |\n{:?}",
                    n_bytes - 4,
                    flags,
                    proto,
                    packet
                );
            }
            Err(e) => {
                println!("Not an IPv4 packet, skipping: {:?}", e);
            }
        }
    }

    Ok(())
}
