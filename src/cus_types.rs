#[derive(Debug, PartialEq)]
pub enum IPv4ProtocolName {
    AH = 0x33,
    ESP = 0x32,
    UDP = 0x11,
    TCP = 0x06,
    ICMP = 0x01,
    IPv6 = 0x29,
    OSPF = 0x59,
    SCTP = 0x84,
}

impl IPv4ProtocolName {
    pub fn from_representation(representation: u8) -> Self {
        match representation {
            0x33 => Self::AH,
            0x32 => Self::ESP,
            0x11 => Self::UDP,
            0x06 => Self::TCP,
            0x01 => Self::ICMP,
            0x29 => Self::IPv6,
            0x59 => Self::OSPF,
            0x84 => Self::SCTP,
            _ => panic!("Unknown IPv4ProtocolName: {:x}", representation),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EtherType {
    SRP = 0x880B,
    ARP = 0x0806,
    IPv6 = 0x86DD,
    IPv4 = 0x0800,
    LLDP = 0x88CC,
    RARP = 0x8035,
    WoLAN = 0x0842,
}

impl EtherType {
    pub fn from_representation(representation: u16) -> Self {
        match representation {
            0x880B => Self::SRP,
            0x0806 => Self::ARP,
            0x86DD => Self::IPv6,
            0x0800 => Self::IPv4,
            0x88CC => Self::LLDP,
            0x8035 => Self::RARP,
            0x0842 => Self::WoLAN,
            _ => panic!("Unknown EtherType: {:x}", representation),
        }
    }
}
