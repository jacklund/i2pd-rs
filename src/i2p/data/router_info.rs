#[derive(Debug, Default)]
pub struct RouterInfo {
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
