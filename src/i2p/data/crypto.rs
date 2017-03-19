#![allow(non_camel_case_types)]

use byteorder::{BigEndian, ReadBytesExt};
use i2p::error::Error;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem;
use std::str;

// macro_rules! generate_crypto {
//     (
//         $t:ident  {
//             $( $name:ident ($x:expr), )+
//         }
//     ) => (
//             pub enum $t {
//                 $($name(Box<[u8]>)),+
//             }

//             impl Debug for $t {
//                 fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//                     match *self {
//                         $(
//                             $t::$name(ref data) => {
//                                 write!(f, "{}::{}({:?})", stringify!($t), stringify!($name), data.to_vec())
//                             }
//                           ),+
//                     }
//                 }
//             }
//     )
// }

pub enum PublicKey {
    ElGamal(Box<[u8]>),
}

impl PublicKey {
    pub fn deserialize<R: Read>(reader: R) -> Result<PublicKey, Error> {
        let mut buffer: [u8; 256] = [0; 256];
        reader.read_exact(&mut buffer)?;

        Ok(PublicKey::ElGamal(box buffer))
    }
}

pub enum PrivateKey {
    ElGamal(Box<[u8]>),
}

pub enum SessionKey {
    ElGamal(Box<[u8]>),
}

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

enum SigningPublicKeyType {
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
            t if t == SigningPublicKeyType::ECDSA_SHA256_P256 as u16 => Ok(SigningPublicKeyType::ECDSA_SHA256_P256),
            t if t == SigningPublicKeyType::ECDSA_SHA384_P384 as u16 => Ok(SigningPublicKeyType::ECDSA_SHA384_P384),
            t if t == SigningPublicKeyType::ECDSA_SHA512_P521 as u16 => Ok(SigningPublicKeyType::ECDSA_SHA512_P521),
            t if t == SigningPublicKeyType::RSA_SHA256_2048 as u16 => Ok(SigningPublicKeyType::RSA_SHA256_2048),
            t if t == SigningPublicKeyType::RSA_SHA384_3072 as u16 => Ok(SigningPublicKeyType::RSA_SHA384_3072),
            t if t == SigningPublicKeyType::RSA_SHA512_4096 as u16 => Ok(SigningPublicKeyType::RSA_SHA512_4096),
            t if t == SigningPublicKeyType::EdDSA_SHA512_Ed25519 as u16 => Ok(SigningPublicKeyType::EdDSA_SHA512_Ed25519),
            t if t == SigningPublicKeyType::EdDSA_SHA512_Ed25519ph as u16 => Ok(SigningPublicKeyType::EdDSA_SHA512_Ed25519ph),
            _ => Err(Error::Crypto("Unknown signing public key type".to_string())),
        }
    }

    pub fn length(key_type: SigningPublicKeyType) -> usize {
        match key_type {
            DSA_SHA1 => 128,
            ECDSA_SHA256_P256 => 64,
            ECDSA_SHA384_P384 => 96,
            ECDSA_SHA512_P521 => 132,
            RSA_SHA256_2048 => 256,
            RSA_SHA384_3072 => 384,
            RSA_SHA512_4096 => 512,
            EdDSA_SHA512_Ed25519 => 32,
            EdDSA_SHA512_Ed25519ph => 32,
        }
    }

    pub fn create_type(key_type: SigningPublicKeyType, data: Box<[u8]>) -> SigningPublicKey {
        match key_type{
            DSA_SHA1 => SigningPublicKey::DSA_SHA1(data),
            ECDSA_SHA256_P256 => SigningPublicKey::ECDSA_SHA256_P256(data),
            ECDSA_SHA384_P384 => SigningPublicKey::ECDSA_SHA384_P384(data),
            ECDSA_SHA512_P521 => SigningPublicKey::ECDSA_SHA512_P521(data),
            RSA_SHA256_2048 => SigningPublicKey::RSA_SHA256_2048(data),
            RSA_SHA384_3072 => SigningPublicKey::RSA_SHA384_3072(data),
            RSA_SHA512_4096 => SigningPublicKey::RSA_SHA512_4096(data),
            EdDSA_SHA512_Ed25519 => SigningPublicKey::EdDSA_SHA512_Ed25519(data),
            EdDSA_SHA512_Ed25519ph => SigningPublicKey::EdDSA_SHA512_Ed25519ph(data),
        }
    }

    pub fn new<R: Read>(key_type: SigningPublicKeyType, reader: R) -> Result<SigningPublicKey, Error> {
        let (padding_size, extra_bytes) = SigningPublicKey::padding_size(key_type)?;
        reader.bytes().skip(padding_size);
        let mut buffer: Vec<u8> = Vec::new();
        reader.take(SigningPublicKey::length(key_type) as u64).read_to_end(&mut buffer);
        Ok(SigningPublicKey::create_type(key_type, buffer.into_boxed_slice()))
    }

    fn padding_size(key_type: SigningPublicKeyType) -> Result<(usize, usize), Error> {
        let size: i32 = 128 - Self::length(key_type) as i32;
        let extra_bytes: i32 = 0;
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

pub enum Certificate {
    Null,
    HashCash(String),
    Hidden,
    Signed(Box<[u8]>),
    Multiple(Box<[u8]>),
    Key(KeyCertificate),
}

struct KeyCertificate {
    signing_key_type: SigningPublicKeyType,
    crypto_key_type: u16,
}

impl KeyCertificate {
    pub fn deserialize<R: Read>(reader: &R) -> Result<KeyCertificate, Error> {
        Ok(KeyCertificate {
            signing_key_type: SigningPublicKey::to_signing_public_key_type(reader.read_u16::<BigEndian>()?)?,
            crypto_key_type: reader.read_u16::<BigEndian>()?,
        })
    }
}

impl Certificate {
    pub fn deserialize<R: Read>(reader: &R) -> Result<Certificate, Error> {
        let cert_type = reader.read_u8()?;
        let length = reader.read_u16::<BigEndian>()?;
        let mut payload: Vec<u8> = Vec::new();
        reader.take(length as u64).read_to_end(&mut payload)?;
        match cert_type {
            0 => Ok(Certificate::Null),
            1 => Ok(Certificate::HashCash(str::from_utf8(payload.as_slice())?.to_string())),
            2 => Ok(Certificate::Hidden),
            3 => Ok(Certificate::Signed(box *payload.as_slice())),
            4 => Ok(Certificate::Multiple(box *payload.as_slice())),
            5 => Ok(Certificate::Key(KeyCertificate::deserialize(reader)?))
        }
    }
}

pub struct KeysAndCert {
    public_key: PublicKey,
    signing_key: SigningPublicKey,
    certificate: Certificate,
}

pub type RouterIdentity = KeysAndCert;

impl KeysAndCert {
    pub fn deserialize<R: Read>(reader: &R) -> Result<KeysAndCert, Error> {
        let mut buffer: [u8; 384];
        reader.read_exact(&mut buffer)?;
        let certificate = Certificate::deserialize(reader)?;
        let signing_key_type = match certificate {
            Certificate::Key(key_cert) => key_cert.signing_key_type,
            _ => SigningPublicKeyType::DSA_SHA1,
        };
        let buffer_reader: &[u8] = &buffer;
        let public_key = PublicKey::deserialize(buffer_reader)?;
        let signing_key = SigningPublicKey::new(signing_key_type, buffer_reader)?;
        Ok( KeysAndCert {
            public_key: public_key,
            signing_key: signing_key,
            certificate: certificate,
        })
    }
}
