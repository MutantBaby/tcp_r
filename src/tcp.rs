pub struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "SRC= {} :{}   -->   DES= {} :{}   LENGTH= {}b of TCP",
            ip_header.source_addr(),
            tcp_header.source_port(),
            ip_header.destination_addr(),
            tcp_header.destination_port(),
            data.len()
        );
    }
}
