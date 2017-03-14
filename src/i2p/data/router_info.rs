use i2p::data::crypto;
use std::collections::HashMap;
use time::Timespec;

pub struct RouterAddress {
    cost: u8,
    expiration: Option<Timespec>,
    transport_style: SupportedTransports,
    options: HashMap<String, String>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct RouterInfo {
    identity: crypto::RouterIdentity,
    published: Timespec,
    addresses: Vec<RouterAddress>,
    options: HashMap<String, String>,
    signature: crypto::Signature,
}

pub enum SupportedTransports {
    NTCPV4,
    NTCPV6,
    SSUV4,
    SSUV6,
}

enum Caps {
    FloodFill = 0x01,
    HighBandwidth = 0x02,
    ExtraBandwidth = 0x04,
    Reachable = 0x08,
    SSUTesting = 0x10,
    SSUIntroducer = 0x20,
    Hidden = 0x40,
    Unreachable = 0x80,
}

enum TransportStyle {
    TransportUnknown = 0,
    TransportNTCP,
    TransportSSU,
}
