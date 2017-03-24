#![allow(non_camel_case_types)]

use byteorder::{BigEndian, ReadBytesExt};
use i2p::error::Error;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::marker::Sized;
use std::mem;
use std::str;

#[derive(Debug)]
pub enum PublicKey {
    ElGamal(Box<[u8]>),
}

impl PublicKey {
    pub fn length(&self) -> usize {
        let PublicKey::ElGamal(ref data) = *self;
        data.len()
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> Result<PublicKey, Error> {
        let mut buffer = vec![0u8; 256];
        let bytes_read = reader.read(buffer.as_mut_slice())?;
        println!("public key buffer = {:?}", buffer);

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
pub enum SigningPublicKey {
    DSA_SHA1(Box<[u8]>),
    ECDSA_SHA256_P256(Box<[u8]>),
    ECDSA_SHA384_P384(Box<[u8]>),
    ECDSA_SHA512_P521(Box<[u8]>),
    RSA_SHA256_2048(Box<[u8]>),
    RSA_SHA384_3072(Box<[u8]>),
    RSA_SHA512_4096(Box<[u8]>),
    EdDSA_SHA512_Ed25519(Box<[u8]>),
    EdDSA_SHA512_Ed25519ph(Box<[u8]>),
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

impl SigningPublicKey {
    pub fn to_signing_public_key_type(t: u16) -> Result<SigningPublicKeyType, Error> {
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
            _ => Err(Error::Crypto("Unknown signing public key type".to_string())),
        }
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
            SigningPublicKeyType::EdDSA_SHA512_Ed25519 => 32,
            SigningPublicKeyType::EdDSA_SHA512_Ed25519ph => 32,
        }
    }

    pub fn create_type(key_type: SigningPublicKeyType, data: Box<[u8]>) -> SigningPublicKey {
        match key_type {
            SigningPublicKeyType::DSA_SHA1 => SigningPublicKey::DSA_SHA1(data),
            SigningPublicKeyType::ECDSA_SHA256_P256 => SigningPublicKey::ECDSA_SHA256_P256(data),
            SigningPublicKeyType::ECDSA_SHA384_P384 => SigningPublicKey::ECDSA_SHA384_P384(data),
            SigningPublicKeyType::ECDSA_SHA512_P521 => SigningPublicKey::ECDSA_SHA512_P521(data),
            SigningPublicKeyType::RSA_SHA256_2048 => SigningPublicKey::RSA_SHA256_2048(data),
            SigningPublicKeyType::RSA_SHA384_3072 => SigningPublicKey::RSA_SHA384_3072(data),
            SigningPublicKeyType::RSA_SHA512_4096 => SigningPublicKey::RSA_SHA512_4096(data),
            SigningPublicKeyType::EdDSA_SHA512_Ed25519 => {
                SigningPublicKey::EdDSA_SHA512_Ed25519(data)
            }
            SigningPublicKeyType::EdDSA_SHA512_Ed25519ph => {
                SigningPublicKey::EdDSA_SHA512_Ed25519ph(data)
            }
        }
    }

    pub fn new<R: Read>(key_type: SigningPublicKeyType,
                        reader: &mut R)
                        -> Result<SigningPublicKey, Error> {
        let (padding_size, extra_bytes) = SigningPublicKey::padding_size(&key_type)?;
        println!("padding size = {:?}", padding_size);
        println!("key length = {:?}", SigningPublicKey::length(&key_type));
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
            println!("Signing public key data = {:?}", data);
            Ok(SigningPublicKey::create_type(key_type, data.into_boxed_slice()))
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
struct KeyCertificate {
    signing_key_type: SigningPublicKeyType,
    crypto_key_type: u16,
    extra_bytes: Vec<u8>,
}

impl KeyCertificate {
    pub fn deserialize<R: Read>(mut reader: R) -> Result<KeyCertificate, Error> {
        let signing_key_type = reader.read_u16::<BigEndian>()?;
        let crypto_key_type = reader.read_u16::<BigEndian>()?;
        let mut extra_bytes: Vec<u8> = Vec::new();
        reader.read_to_end(&mut extra_bytes)?;
        Ok(KeyCertificate {
            signing_key_type:
                SigningPublicKey::to_signing_public_key_type(signing_key_type)?,
            crypto_key_type: crypto_key_type,
            extra_bytes: extra_bytes,
        })
    }
}

impl Certificate {
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Certificate, Error> {
        let cert_type = reader.read_u8()?;
        let length = reader.read_u16::<BigEndian>()?;
        let mut payload = vec![0u8; length as usize];
        let bytes_read = reader.read(payload.as_mut_slice())?;
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

pub type RouterIdentity = KeysAndCert;

impl KeysAndCert {
    pub fn deserialize<R: Read>(mut reader: R) -> Result<KeysAndCert, Error> {
        let mut buffer = vec![0u8; 384];
        let size_read = reader.read(buffer.as_mut_slice())?;
        println!("Keys and cert buffer = {:?}", buffer);
        let certificate = Certificate::deserialize(&mut reader)?;
        let mut signing_key_type = SigningPublicKeyType::DSA_SHA1;
        println!("signing key type = {:?}", signing_key_type);
        let mut extra_bytes: Option<Vec<u8>> = None;
        if let Certificate::Key(ref key_cert) = certificate {
            signing_key_type = key_cert.signing_key_type.clone();
            buffer.extend(key_cert.extra_bytes.clone());
        }
        let mut reader = buffer.as_slice();
        let public_key = PublicKey::deserialize(&mut reader)?;
        let signing_key = SigningPublicKey::new(signing_key_type, &mut reader)?;
        Ok(KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: certificate,
        })
    }
}

#[cfg(test)]
mod test {
    use base64::{decode, encode};
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use super::*;

    fn read_fixture(filename: &str) -> Vec<String> {
        let file_path = format!("fixtures/{}.txt", filename);
        let mut file = match File::open(&file_path) {
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
                    if len == 0 { break; }
                    ret.push(buffer);
                    buffer = String::new();
                }
            }
        }

        ret
    }

    #[test]
    fn test_deserialize_DSA_SHA1_keys_and_cert() {
        let values = read_fixture("DSA_SHA1_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::DSA_SHA1(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        assert_eq!(Certificate::Null, keys_and_cert.certificate);
    }

    #[test]
    fn test_deserialize_ECDSA_SHA256_P256_keys_and_cert() {
        let values = read_fixture("ECDSA_SHA256_P256_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::ECDSA_SHA256_P256(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::ECDSA_SHA256_P256);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_deserialize_ECDSA_SHA384_P384_keys_and_cert() {
        let values = read_fixture("ECDSA_SHA384_P384_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::ECDSA_SHA384_P384(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::ECDSA_SHA384_P384);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_deserialize_ECDSA_SHA512_P521_keys_and_cert() {
        let values = read_fixture("ECDSA_SHA512_P521_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        println!("result = {:?}", result);
        println!("result length = {:?}", result.len());
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::ECDSA_SHA512_P521(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::ECDSA_SHA512_P521);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_deserialize_EdDSA_SHA512_Ed25519_keys_and_cert() {
        let values = read_fixture("EdDSA_SHA512_Ed25519_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        println!("result = {:?}", result);
        println!("result length = {:?}", result.len());
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::EdDSA_SHA512_Ed25519(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::EdDSA_SHA512_Ed25519);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_deserialize_EdDSA_SHA512_Ed25519ph_keys_and_cert() {
        let values = read_fixture("EdDSA_SHA512_Ed25519ph_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        println!("result = {:?}", result);
        println!("result length = {:?}", result.len());
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::EdDSA_SHA512_Ed25519ph(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::EdDSA_SHA512_Ed25519ph);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_deserialize_RSA_SHA256_2048_keys_and_cert() {
        let values = read_fixture("RSA_SHA256_2048_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        println!("result = {:?}", result);
        println!("result length = {:?}", result.len());
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::RSA_SHA256_2048(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::RSA_SHA256_2048);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_deserialize_RSA_SHA384_3072_keys_and_cert() {
        let values = read_fixture("RSA_SHA384_3072_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::RSA_SHA384_3072(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::RSA_SHA384_3072);
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_deserialize_RSA_SHA512_4096_keys_and_cert() {
        let values = read_fixture("RSA_SHA512_4096_Keys_and_Cert");

        let mut decode_result = decode(&values[0].trim_right());
        let result = decode_result.unwrap();
        let keys_and_cert_result = KeysAndCert::deserialize(result.as_slice());
        assert!(keys_and_cert_result.is_ok());
        let keys_and_cert = keys_and_cert_result.unwrap();

        decode_result = decode(&values[1].trim_right());
        let public_key_data = decode_result.unwrap();
        match keys_and_cert.public_key {
            PublicKey::ElGamal(data) => {
                assert_eq!(*public_key_data.as_slice(), *data);
            }
        };
        decode_result = decode(&values[2].trim_right());
        let signing_key_data = decode_result.unwrap();
        match keys_and_cert.signing_key {
            SigningPublicKey::RSA_SHA512_4096(data) => assert_eq!(*signing_key_data.as_slice(), *data),
            _ => assert!(false),
        }
        match keys_and_cert.certificate {
            Certificate::Key(key_cert) => {
                assert_eq!(key_cert.crypto_key_type, 0);
                assert_eq!(key_cert.signing_key_type, SigningPublicKeyType::RSA_SHA512_4096);
            },
            _ => assert!(false),
        }
    }
}