use std::io::Result;

use tun_tap::{Iface, Mode};

extern crate tun_tap;

fn main() -> Result<()> {
    let iface0: Iface = Iface::new("tun0", Mode::Tun)?;
    let mut buff: Vec<u8> = vec![0; 1504];

    let n_bytes: usize = iface0.recv(&mut buff)?;

    println!("Received {} bytes |\n{:x?}", n_bytes, &buff[..n_bytes]);
    Ok(())
}
