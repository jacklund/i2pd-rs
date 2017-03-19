use byteorder::NetworkEndian;
use i2p::error::Error;
use serde::{Deserializer, Serializer};
use std::io::{BufReader, BufWriter, Read, Write};

pub struct I2PSerializer(pub BufWriter);
pub struct I2PDeserializer(pub BufReader);

impl I2PSerializer {
    pub fn new(write: Write) -> I2PSerializer {
        I2PSerializer(BufWriter::new(write))
    }

    pub fn push(&mut self, byte: u8) {
        self.0.write_all(&byte);
    }

    pub fn append(&mut self, data: &mut Vec<u8>) {
    }
}

impl Serializer for I2PSerializer {
    type Ok = ();
    type Error = Error::Serialization;

    fn serialize_u8(self, v: u8) -> Result(Self::Ok, Self::Error) {
        self.push(v)
    }

    fn serialize_u16(self, v: u16) -> Result(Self::Ok, Self::Error) {

    }
}