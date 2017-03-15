use i2p::data::crypto;
use rustc_serialize::{Decodable, Decoder, DecoderHelpers, Encodable, Encoder};
use std::collections::HashMap;

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct RouterAddress {
    cost: u8,
    expiration: Option<u64>,
    transport_style: SupportedTransports,
    options: HashMap<String, String>,
}

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct RouterInfo {
    identity: crypto::RouterIdentity,
    published: u64,
    addresses: Vec<RouterAddress>,
    options: HashMap<String, String>,
    signature: crypto::Signature,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
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
    TransportUnknown = 0,
    TransportNTCP,
    TransportSSU,
}
