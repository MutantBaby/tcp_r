use std::io::Result;

use tun_tap::{Iface, Mode};

fn main() -> Result<()> {
    let iface0: Iface = Iface::new("tun0", Mode::Tun)?;
    let mut buff: Vec<u8> = vec![0; 1504];

    loop {
        let n_bytes: usize = iface0.recv(&mut buff)?;
        let _eth_flags: u16 = u16::from_be_bytes([buff[0], buff[1]]);
        let eth_proto: u16 = u16::from_be_bytes([buff[2], buff[3]]);

        if eth_proto != 0x0800 {
            // not IPv4
            println!("Not an IPv4 packet, skipping");
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buff[4..n_bytes]) {
            Ok(packet) => {
                // if packet.protocol() != 0x06 {
                //     // not TCP
                //     println!("Not TCP packet, skipping");
                //     continue;
                // }

                match etherparse::TcpHeaderSlice::from_slice(&buff[4 + packet.slice().len()..]) {
                    Ok(tcp) => {
                        println!(
                            "SRC: {}:{}  ->  DES: {}:{}  LENGTH: {}b of TCP",
                            packet.source_addr(),
                            tcp.source_port(),
                            packet.destination_addr(),
                            tcp.destination_port(),
                            packet.payload_len()
                        );
                    }
                    Err(e) => {
                        println!("Not a TCP packet, skipping: {:?}", e);
                    }
                }

                // eprintln!(
                //     "SRC: {}  ->  DES: {}  LENGTH: {}b of PROTOCOL: {}",
                //     packet.source_addr(),
                //     packet.destination_addr(),
                //     packet.payload_len(),
                //     packet.protocol()
                // );
            }
            Err(e) => {
                println!("Not an IPv4 packet, skipping: {:?}", e);
            }
        }
    }
}
