use i2p::data::crypto;
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, RustcEncodable, RustcDecodable)]
pub struct RouterAddress {
    pub cost: u8,
    pub expiration: Option<u64>,
    pub transport_style: SupportedTransports,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct RouterInfo {
    identity: crypto::RouterIdentity,
    published: u64,
    addresses: Vec<RouterAddress>,
    options: HashMap<String, String>,
    signature: crypto::Signature,
}

#[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub enum SupportedTransports {
    NTCPV4,
    NTCPV6,
    SSUV4,
    SSUV6,
}

impl Default for SupportedTransports {
    fn default() -> SupportedTransports {
        SupportedTransports::NTCPV4
    }
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
    Unknown = 0,
    NTCP,
    SSU,
}
