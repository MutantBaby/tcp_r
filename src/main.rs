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
        
        println!("Received {} bytes |\n{:x?}", n_bytes - 4, &buff[4..n_bytes]);
    }

    Ok(())
}
