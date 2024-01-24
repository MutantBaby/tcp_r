use std::{collections::HashMap, io::Result, net::Ipv4Addr};
use tun_tap::{Iface, Mode};

mod tcp;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> Result<()> {
    
    let mut connection: HashMap<Quad, tcp::State> = Default::default();
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
            Ok(ip_header) => {
                if ip_header.protocol() != 0x06 {
                    // not TCP
                    // eprintln!("Not TCP packet, skipping");
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buff[4 + ip_header.slice().len()..]) {
                    Ok(tcp_header) => {
                        let data_i: usize = 4 + ip_header.slice().len() + tcp_header.slice().len();

                        connection
                            .entry(Quad {
                                src: (ip_header.source_addr(), tcp_header.source_port()),
                                dst: (ip_header.destination_addr(), tcp_header.destination_port()),
                            })
                            .or_default()
                            .on_packet(ip_header.clone(), tcp_header.clone(), &buff[data_i..]);

                        // println!(
                        //     "SRC: {}:{}  ->  DES: {}:{}  LENGTH: {}b of TCP",
                        //     ip_header.source_addr(),
                        //     tcp_header.source_port(),
                        //     ip_header.destination_addr(),
                        //     tcp_header.destination_port(),
                        //     ip_header.payload_len()
                        // );
                    }
                    Err(e) => {
                        eprintln!("Not a TCP packet, skipping: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Not an IPv4 packet, skipping: {:?}", e);
            }
        }
    }
}
