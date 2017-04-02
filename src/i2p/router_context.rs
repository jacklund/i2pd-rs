use i2p::config::Config;
use i2p::crypto;
use i2p::data::router_info::RouterInfo;
use i2p::error::Error;
use std::default::Default;

const I2PD_NET_ID: &str = "2";

#[derive(Default)]
pub struct RouterContext {
    net_id: String,
    ipv4: bool,
    ipv6: bool,
    port: u32,
    accepts_tunnels: bool,
    max_num_transit_tunnels: u32,
    flood_fill: bool,
    bandwidth_limit: u32,
    bandwidth_type: Option<BandwidthType>,
    router_info: Option<RouterInfo>,
}

#[derive(Debug)]
enum BandwidthType {
    Low,
    High,
    Extra,
    Unlimited,
}

impl RouterContext {
    pub fn new(config: &Config) -> Result<RouterContext, Error> {
        let mut context: RouterContext = Default::default();
        context.initialize(config)?;

        Ok(context)
    }

    fn initialize(&mut self, config: &Config) -> Result<(), Error> {
        self.net_id = config.string_value("netid", Some(I2PD_NET_ID)).unwrap();
        if self.net_id != I2PD_NET_ID {
            crypto::init_gost();  // Init GOST for our own darknet
        }

        self.configure_ipv4(config.bool_value("ipv4", Some(true)).unwrap());
        self.configure_ipv6(config.bool_value("ipv6", Some(true)).unwrap());

        self.update_port(config.i64_value("port", Some(0)).unwrap() as u32);

        self.accepts_tunnels = !config.bool_value("notransit", Some(false)).unwrap();
        self.set_max_num_transit_tunnels(config.i64_value("limits.transittunnels", Some(2500))
                .unwrap() as u32)?;
        self.set_flood_fill(config.bool_value("floodfill", Some(false)).unwrap())?;
        self.set_bandwidth(config.string_value("bandwidth", None).as_ref().map(|s| &**s))?;
        self.set_family(config.string_value("family", None).as_ref().map(|s| &**s))?;

        Ok(())
    }

    fn set_family(&mut self, family: Option<&str>) -> Result<(), Error> {
        if family.is_some() {
            // Create family signature and set the family and signature in router info
            unimplemented!()
        }

        Ok(())
    }

    fn set_bandwidth(&mut self, bandwidth: Option<&str>) -> Result<(), Error> {
        match bandwidth {
            Some(bandwidth) => {
                match bandwidth {
                    "K" => {
                        self.bandwidth_limit = 12;
                        self.bandwidth_type = Some(BandwidthType::Low);
                    }
                    "L" => {
                        self.bandwidth_limit = 48;
                        self.bandwidth_type = Some(BandwidthType::Low);
                    }
                    "M" => {
                        self.bandwidth_limit = 64;
                        self.bandwidth_type = Some(BandwidthType::High);
                    }
                    "N" => {
                        self.bandwidth_limit = 128;
                        self.bandwidth_type = Some(BandwidthType::High);
                    }
                    "O" => {
                        self.bandwidth_limit = 256;
                        self.bandwidth_type = Some(BandwidthType::High);
                    }
                    "P" => {
                        self.bandwidth_limit = 2048;
                        self.bandwidth_type = Some(BandwidthType::Extra);
                    }
                    "X" => {
                        self.bandwidth_limit = 9999;
                        self.bandwidth_type = Some(BandwidthType::Unlimited);
                    }
                    _ => {
                        let value = bandwidth.parse::<u32>()?;
                        if value > 2000 {
                            self.set_bandwidth(Some("X"))?;
                        } else if value > 256 {
                            self.set_bandwidth(Some("P"))?;
                        } else if value > 128 {
                            self.set_bandwidth(Some("O"))?;
                        } else if value > 64 {
                            self.set_bandwidth(Some("N"))?;
                        } else if value > 48 {
                            self.set_bandwidth(Some("M"))?;
                        } else if value > 12 {
                            self.set_bandwidth(Some("L"))?;
                        } else {
                            self.set_bandwidth(Some("K"))?;
                        }
                    }
                }
            }
            None => {
                if self.flood_fill {
                    self.set_bandwidth(Some("P"))?;
                } else {
                    self.set_bandwidth(Some("L"))?;
                }
            }
        };

        Ok(())
    }

    fn set_flood_fill(&mut self, flood_fill: bool) -> Result<(), Error> {
        self.flood_fill = flood_fill;

        Ok(())
    }

    fn set_max_num_transit_tunnels(&mut self, max_tunnels: u32) -> Result<(), Error> {
        self.max_num_transit_tunnels = max_tunnels;

        Ok(())
    }

    fn update_port(&mut self, port: u32) {
        self.port = port;
    }

    fn configure_ipv4(&mut self, ipv4: bool) {
        self.ipv4 = ipv4;
    }

    fn configure_ipv6(&mut self, ipv6: bool) {
        self.ipv6 = ipv6;
    }
}