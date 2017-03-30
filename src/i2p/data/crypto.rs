#![allow(non_camel_case_types)]

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use i2p::error::Error;
use rand::{OsRng, Rand, Rng};
use std::io::{self, Read, Write};
use std::str;

#[derive(Debug)]
pub enum PublicKey {
    ElGamal(Box<[u8]>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PublicKeyType {
    ElGamal,
}

impl PublicKeyType {
    pub fn from_u16(t: u16) -> Result<PublicKeyType, Error> {
        match t {
            t if t == PublicKeyType::ElGamal as u16 => Ok(PublicKeyType::ElGamal),
            _ => Err(Error::Crypto(format!("Unknown public key type"))),
        }
    }
}

impl PublicKey {
    pub fn get_type(&self) -> PublicKeyType {
        match *self {
            PublicKey::ElGamal(_) => PublicKeyType::ElGamal,
        }
    }

    pub fn length(&self) -> usize {
        let PublicKey::ElGamal(ref data) = *self;
        data.len()
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        let PublicKey::ElGamal(ref data) = *self;
        Ok(writer.write(data.as_ref())?)
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> Result<PublicKey, Error> {
        let mut buffer = vec![0u8; 256];
        reader.read_exact(buffer.as_mut_slice())?;

        Ok(PublicKey::ElGamal(buffer.into_boxed_slice()))
    }
}

pub enum PrivateKey {
    ElGamal(Box<[u8]>),
}

pub enum SessionKey {
    ElGamal(Box<[u8]>),
}

#[derive(Debug)]
pub struct SigningPublicKey {
    key_type: SigningPublicKeyType,
    data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SigningPublicKeyType {
    DSA_SHA1,
    ECDSA_SHA256_P256,
    ECDSA_SHA384_P384,
    ECDSA_SHA512_P521,
    RSA_SHA256_2048,
    RSA_SHA384_3072,
    RSA_SHA512_4096,
    EdDSA_SHA512_Ed25519,
    EdDSA_SHA512_Ed25519ph,
}

impl SigningPublicKeyType {
    pub fn from_u16(t: u16) -> Result<SigningPublicKeyType, Error> {
        match t {
            t if t == SigningPublicKeyType::DSA_SHA1 as u16 => Ok(SigningPublicKeyType::DSA_SHA1),
            t if t == SigningPublicKeyType::ECDSA_SHA256_P256 as u16 => {
                Ok(SigningPublicKeyType::ECDSA_SHA256_P256)
            }
            t if t == SigningPublicKeyType::ECDSA_SHA384_P384 as u16 => {
                Ok(SigningPublicKeyType::ECDSA_SHA384_P384)
            }
            t if t == SigningPublicKeyType::ECDSA_SHA512_P521 as u16 => {
                Ok(SigningPublicKeyType::ECDSA_SHA512_P521)
            }
            t if t == SigningPublicKeyType::RSA_SHA256_2048 as u16 => {
                Ok(SigningPublicKeyType::RSA_SHA256_2048)
            }
            t if t == SigningPublicKeyType::RSA_SHA384_3072 as u16 => {
                Ok(SigningPublicKeyType::RSA_SHA384_3072)
            }
            t if t == SigningPublicKeyType::RSA_SHA512_4096 as u16 => {
                Ok(SigningPublicKeyType::RSA_SHA512_4096)
            }
            t if t == SigningPublicKeyType::EdDSA_SHA512_Ed25519 as u16 => {
                Ok(SigningPublicKeyType::EdDSA_SHA512_Ed25519)
            }
            t if t == SigningPublicKeyType::EdDSA_SHA512_Ed25519ph as u16 => {
                Ok(SigningPublicKeyType::EdDSA_SHA512_Ed25519ph)
            }
            _ => Err(Error::Crypto(format!("Unknown signing public key type"))),
        }
    }
}

impl SigningPublicKey {
    pub fn get_type(&self) -> SigningPublicKeyType {
        self.key_type.clone()
    }

    pub fn length(key_type: &SigningPublicKeyType) -> usize {
        match *key_type {
            SigningPublicKeyType::DSA_SHA1 => 128,
            SigningPublicKeyType::ECDSA_SHA256_P256 => 64,
            SigningPublicKeyType::ECDSA_SHA384_P384 => 96,
            SigningPublicKeyType::ECDSA_SHA512_P521 => 132,
            SigningPublicKeyType::RSA_SHA256_2048 => 256,
            SigningPublicKeyType::RSA_SHA384_3072 => 384,
            SigningPublicKeyType::RSA_SHA512_4096 => 512,
            SigningPublicKeyType::EdDSA_SHA512_Ed25519 |
            SigningPublicKeyType::EdDSA_SHA512_Ed25519ph => 32,
        }
    }

    pub fn new(key_type: SigningPublicKeyType, data: &[u8]) -> SigningPublicKey {
        SigningPublicKey {
            key_type: key_type,
            data: data.to_vec(),
        }
    }

    pub fn serialize<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        Ok(writer.write(&self.data)?)
    }

    pub fn deserialize<R: Read>(key_type: SigningPublicKeyType,
                                reader: &mut R)
                                -> Result<SigningPublicKey, Error> {
        let (padding_size, _) = SigningPublicKey::padding_size(&key_type)?;
        let result: Result<Vec<u8>, io::Error> =
            reader.bytes().skip(padding_size).take(SigningPublicKey::length(&key_type)).collect();
        if result.is_err() {
            return Err(Error::from(result.unwrap_err()));
        }
        let data = result.unwrap();
        if data.len() != Self::length(&key_type) {
            Err(Error::Crypto(format!("Expected signing public key of length {}, got one of \
                                       length {}",
                                    Self::length(&key_type),
                                    data.len())))
        } else {
            Ok(SigningPublicKey::new(key_type, &data))
        }
    }

    fn padding_size(key_type: &SigningPublicKeyType) -> Result<(usize, usize), Error> {
        let mut size: i32 = 128 - Self::length(key_type) as i32;
        let mut extra_bytes: i32 = 0;
        if size < 0 {
            extra_bytes = -(size as i32);
            size = 0;
        }

        Ok((size as usize, extra_bytes as usize))
    }
}

pub enum SigningPrivateKey {
    DSA_SHA1(Box<[u8]>), // length = 20
    ECDSA_SHA256_P256(Box<[u8]>), // length = 32
    ECDSA_SHA384_P384(Box<[u8]>), // length = 48
    ECDSA_SHA512_P521(Box<[u8]>), // length = 66
    RSA_SHA256_2048(Box<[u8]>), // length = 512
    RSA_SHA384_3072(Box<[u8]>), // length = 768
    RSA_SHA512_4096(Box<[u8]>), // length = 1024
    EdDSA_SHA512_Ed25519(Box<[u8]>), // length = 32
    EdDSA_SHA512_Ed25519ph(Box<[u8]>), // length = 32
}

pub enum Signature {
    DSA_SHA1(Box<[u8]>), // length = 40
    ECDSA_SHA256_P256(Box<[u8]>), // length = 64
    ECDSA_SHA384_P384(Box<[u8]>), // length = 96
    ECDSA_SHA512_P521(Box<[u8]>), // length = 132
    RSA_SHA256_2048(Box<[u8]>), // length = 256
    RSA_SHA384_3072(Box<[u8]>), // length = 384
    RSA_SHA512_4096(Box<[u8]>), // length = 512
    EdDSA_SHA512_Ed25519(Box<[u8]>), // length = 64
    EdDSA_SHA512_Ed25519ph(Box<[u8]>), // length = 64
}

pub enum Hash {
    SHA256(Box<[u8]>), // length = 32
}

pub enum CertificateType {
    Null = 0,
    HashCash = 1,
    Hidden = 2,
    Signed = 3,
    Multiple = 4,
    Key = 5,
}

#[derive(Debug, PartialEq)]
pub enum Certificate {
    Null,
    HashCash(String),
    Hidden,
    Signed(Box<[u8]>),
    Multiple(Box<[u8]>),
    Key(KeyCertificate),
}

#[derive(Debug, PartialEq)]
pub struct KeyCertificate {
    signing_key_type: SigningPublicKeyType,
    crypto_key_type: PublicKeyType,
    extra_bytes: Vec<u8>,
}

impl KeyCertificate {
    pub fn new(public_key: &PublicKey,
               signing_key: &SigningPublicKey)
               -> Result<KeyCertificate, Error> {
        let (_, extra_bytes) = SigningPublicKey::padding_size(&signing_key.get_type())?;
        Ok(KeyCertificate {
            crypto_key_type: public_key.get_type(),
            signing_key_type: signing_key.get_type(),
            extra_bytes: signing_key.data[signing_key.data.len() - extra_bytes..].to_vec(),
        })
    }

    pub fn deserialize<R: Read>(mut reader: R) -> Result<KeyCertificate, Error> {
        let signing_key_type = reader.read_u16::<BigEndian>()?;
        let crypto_key_type = reader.read_u16::<BigEndian>()?;
        let mut extra_bytes: Vec<u8> = Vec::new();
        reader.read_to_end(&mut extra_bytes)?;
        Ok(KeyCertificate {
            signing_key_type: SigningPublicKeyType::from_u16(signing_key_type)?,
            crypto_key_type: PublicKeyType::from_u16(crypto_key_type)?,
            extra_bytes: extra_bytes,
        })
    }

    pub fn serialize<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
        writer.write_u16::<BigEndian>(self.signing_key_type.clone() as u16)?;
        writer.write_u16::<BigEndian>(self.crypto_key_type.clone() as u16)?;
        let mut written: usize = 4;
        written += writer.write(&self.extra_bytes)?;

        Ok(written)
    }
}

impl Certificate {
    pub fn serialize<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
        let mut written: usize = 0;
        match *self {
            Certificate::Null => {
                writer.write_u8(CertificateType::Null as u8)?;
                writer.write_u16::<BigEndian>(0 as u16)?;
                written += 3;
            }
            Certificate::HashCash(ref data) => {
                writer.write_u8(CertificateType::HashCash as u8)?;
                writer.write_u16::<BigEndian>(data.len() as u16)?;
                written += 3;
                written += writer.write(data.as_bytes())?;
            }
            Certificate::Hidden => {
                writer.write_u8(CertificateType::Hidden as u8)?;
                writer.write_u16::<BigEndian>(0 as u16)?;
                written += 3;
            }
            Certificate::Signed(ref data) => {
                writer.write_u8(CertificateType::Signed as u8)?;
                writer.write_u16::<BigEndian>(data.len() as u16)?;
                written += 3;
                written += writer.write(data.as_ref())?;
            }
            Certificate::Multiple(ref data) => {
                writer.write_u8(CertificateType::Multiple as u8)?;
                writer.write_u16::<BigEndian>(data.len() as u16)?;
                written += 3;
                written += writer.write(data.as_ref())?;
            }
            Certificate::Key(ref key_cert) => {
                writer.write_u8(CertificateType::Key as u8)?;
                written += 1;
                let mut buffer: Vec<u8> = Vec::new();
                key_cert.serialize(&mut buffer)?;
                writer.write_u16::<BigEndian>(buffer.len() as u16)?;
                written += 2;
                written += writer.write(&buffer)?;
            }
        }

        Ok(written)
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Certificate, Error> {
        let cert_type = reader.read_u8()?;
        let length = reader.read_u16::<BigEndian>()?;
        let mut payload = vec![0u8; length as usize];
        reader.read(payload.as_mut_slice())?;
        match cert_type {
            0 => Ok(Certificate::Null),
            1 => Ok(Certificate::HashCash(str::from_utf8(payload.as_slice())?.to_string())),
            2 => Ok(Certificate::Hidden),
            3 => Ok(Certificate::Signed(payload.into_boxed_slice())),
            4 => Ok(Certificate::Multiple(payload.into_boxed_slice())),
            5 => Ok(Certificate::Key(KeyCertificate::deserialize(payload.as_slice())?)),
            _ => Err(Error::Crypto(format!("Unexpected cert type {} found", cert_type))),
        }
    }
}

#[derive(Debug)]
pub struct KeysAndCert {
    public_key: PublicKey,
    signing_key: SigningPublicKey,
    certificate: Certificate,
}

struct Random {
    rng: OsRng,
}

impl Random {
    pub fn new() -> Result<Random, Error> {
        let rng = OsRng::new()?;
        Ok(Random { rng: rng })
    }

    fn generate<T: Rand>(&mut self, length: usize) -> Vec<T> {
        self.rng.gen_iter::<T>().take(length).collect::<Vec<T>>()
    }
}

pub type RouterIdentity = KeysAndCert;

impl KeysAndCert {
    pub fn serialize<W: Write>(&mut self, mut writer: W) -> Result<usize, Error> {
        let mut written = self.public_key.serialize(&mut writer)?;
        let mut buffer: Vec<u8> = Vec::new();
        self.signing_key.serialize(&mut buffer)?;
        let mut signing_key_type = SigningPublicKeyType::DSA_SHA1;
        let mut padding_size: usize = 0;
        let mut extra_bytes: usize = 0;
        if let Certificate::Key(ref mut key_cert) = self.certificate {
            signing_key_type = key_cert.signing_key_type.clone();
            let (pad, extra) = SigningPublicKey::padding_size(&signing_key_type)?;
            padding_size = pad;
            extra_bytes = extra;
            key_cert.extra_bytes = buffer[buffer.len() - extra_bytes..].to_vec();
        }
        let padding = Random::new()?.generate::<u8>(padding_size);
        written += writer.write(padding.as_slice())?;
        written += writer.write(&buffer[..buffer.len() - extra_bytes])?;
        written += self.certificate.serialize(writer)?;

        Ok(written)
    }

    pub fn deserialize<R: Read>(mut reader: R) -> Result<KeysAndCert, Error> {
        let mut buffer = vec![0u8; 384];
        reader.read_exact(buffer.as_mut_slice())?;
        let certificate = Certificate::deserialize(&mut reader)?;
        let mut signing_key_type = SigningPublicKeyType::DSA_SHA1;
        if let Certificate::Key(ref key_cert) = certificate {
            signing_key_type = key_cert.signing_key_type.clone();
            buffer.extend(key_cert.extra_bytes.clone());
        }
        let mut reader = buffer.as_slice();
        let public_key = PublicKey::deserialize(&mut reader)?;
        let signing_key = SigningPublicKey::deserialize(signing_key_type, &mut reader)?;
        Ok(KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: certificate,
        })
    }
}

#[cfg(test)]
mod test {
    #![allow(non_camel_case_types)]

    use base64::{decode, encode};
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use super::*;

    fn read_fixture(name: &str) -> Vec<String> {
        let file_path = format!("fixtures/{}.txt", name);
        let file = match File::open(&file_path) {
            Err(why) => panic!("couldn't open {}: {}", file_path, why.description()),
            Ok(file) => file,
        };

        let mut reader = BufReader::new(file);
        let mut ret: Vec<String> = Vec::new();
        let mut buffer = String::new();
        loop {
            match reader.read_line(&mut buffer) {
                Err(why) => panic!("couldn't read {}: {}", file_path, why.description()),
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    ret.push(buffer);
                    buffer = String::new();
                }
            }
        }

        ret
    }

    fn get_key_and_cert_fixture_data(name: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let values = read_fixture(name);

        let mut decode_result = decode(&values[0].trim_right());
        let keys_and_cert_data = decode_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();

        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();

        (keys_and_cert_data, public_key_data, signing_key_data)
    }

    #[test]
    fn test_serialize_DSA_SHA1_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("DSA_SHA1_Keys_and_Cert");

        let mut keys_and_cert = KeysAndCert {
            public_key: PublicKey::ElGamal(public_key_data.into_boxed_slice()),
            signing_key: SigningPublicKey {
                key_type: SigningPublicKeyType::DSA_SHA1,
                data: signing_key_data,
            },
            certificate: Certificate::Null,
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        assert_eq!(keys_and_cert_data[..size], buffer[..]);
    }

    #[test]
    fn test_deserialize_DSA_SHA1_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("DSA_SHA1_Keys_and_Cert");
        let values = read_fixture("DSA_SHA1_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::DSA_SHA1,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        assert_eq!(Certificate::Null, keys_and_cert.certificate);
    }

    #[test]
    fn test_serialize_ECDSA_SHA256_P256_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("ECDSA_SHA256_P256_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::ECDSA_SHA256_P256,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::ECDSA_SHA256_P256).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }

    #[test]
    fn test_deserialize_ECDSA_SHA256_P256_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("ECDSA_SHA256_P256_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::ECDSA_SHA256_P256,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::ECDSA_SHA256_P256);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_serialize_ECDSA_SHA384_P384_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("ECDSA_SHA384_P384_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::ECDSA_SHA384_P384,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::ECDSA_SHA384_P384).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }

    #[test]
    fn test_deserialize_ECDSA_SHA384_P384_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("ECDSA_SHA384_P384_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::ECDSA_SHA384_P384,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::ECDSA_SHA384_P384);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_serialize_ECDSA_SHA512_P521_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("ECDSA_SHA512_P521_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::ECDSA_SHA512_P521,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::ECDSA_SHA512_P521).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }

    #[test]
    fn test_deserialize_ECDSA_SHA512_P521_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("ECDSA_SHA512_P521_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::ECDSA_SHA512_P521,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::ECDSA_SHA512_P521);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_serialize_EdDSA_SHA512_Ed25519_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("EdDSA_SHA512_Ed25519_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::EdDSA_SHA512_Ed25519,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::EdDSA_SHA512_Ed25519).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }

    #[test]
    fn test_deserialize_EdDSA_SHA512_Ed25519_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("EdDSA_SHA512_Ed25519_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::EdDSA_SHA512_Ed25519,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::EdDSA_SHA512_Ed25519);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_serialize_EdDSA_SHA512_Ed25519ph_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("EdDSA_SHA512_Ed25519ph_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::EdDSA_SHA512_Ed25519ph,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::EdDSA_SHA512_Ed25519ph).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }

    #[test]
    fn test_deserialize_EdDSA_SHA512_Ed25519ph_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("EdDSA_SHA512_Ed25519ph_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::EdDSA_SHA512_Ed25519ph,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::EdDSA_SHA512_Ed25519ph);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_serialize_RSA_SHA256_2048_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("RSA_SHA256_2048_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::RSA_SHA256_2048,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::RSA_SHA256_2048).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }


    #[test]
    fn test_deserialize_RSA_SHA256_2048_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("RSA_SHA256_2048_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::RSA_SHA256_2048,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::RSA_SHA256_2048);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_serialize_RSA_SHA384_3072_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("RSA_SHA384_3072_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::RSA_SHA384_3072,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::RSA_SHA384_3072).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }

    #[test]
    fn test_deserialize_RSA_SHA384_3072_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("RSA_SHA384_3072_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::RSA_SHA384_3072,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::RSA_SHA384_3072);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_serialize_RSA_SHA512_4096_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("RSA_SHA512_4096_Keys_and_Cert");

        let public_key = PublicKey::ElGamal(public_key_data.into_boxed_slice());
        let signing_key = SigningPublicKey::new(SigningPublicKeyType::RSA_SHA512_4096,
                                                &signing_key_data);
        let key_cert = KeyCertificate::new(&public_key, &signing_key).unwrap();
        let mut keys_and_cert = KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: Certificate::Key(key_cert),
        };

        let mut buffer: Vec<u8> = Vec::new();
        let size = keys_and_cert.serialize(&mut buffer).unwrap();
        let (padding_size, _) =
            SigningPublicKey::padding_size(&SigningPublicKeyType::RSA_SHA512_4096).unwrap();
        assert_eq!(keys_and_cert_data[..256], buffer[..256]);
        assert_eq!(keys_and_cert_data[256 + padding_size..size],
                   buffer[256 + padding_size..]);
    }

    #[test]
    fn test_deserialize_RSA_SHA512_4096_keys_and_cert() {
        let (keys_and_cert_data, public_key_data, signing_key_data) =
            get_key_and_cert_fixture_data("RSA_SHA512_4096_Keys_and_Cert");

        let keys_and_cert_result = KeysAndCert::deserialize(keys_and_cert_data.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        assert_eq!(SigningPublicKeyType::RSA_SHA512_4096,
                   keys_and_cert.signing_key.key_type);
        assert_eq!(signing_key_data, keys_and_cert.signing_key.data);
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, PublicKeyType::ElGamal);
                assert_eq!(key_cert.signing_key_type,
                           SigningPublicKeyType::RSA_SHA512_4096);
            }
            _ => assert!(false),
        }
    }
}