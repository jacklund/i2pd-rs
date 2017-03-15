#![allow(non_camel_case_types)]

use rustc_serialize::{Decodable, Decoder, DecoderHelpers, Encodable, Encoder};
use std::fmt::{self, Debug, Formatter};

macro_rules! generate_crypto {
    (
        $t:ident  {
            $( $name:ident ($x:expr) = $idx:expr, )+
        }
    ) => (
            pub enum $t {
                $($name(Box<[u8; $x]>)),+
            }

            impl Debug for $t {
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    match *self {
                        $(
                            $t::$name(ref data) => {
                                write!(f, "{}::{}({:?})", stringify!($t), stringify!($name), data.to_vec())
                            }
                          ),+
                    }
                }
            }

            impl Encodable for $t {
                fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
                    match *self {
                        $( $t::$name(ref data) => {
                            let id: u8 = $idx;
                            id.encode(s)?;
                            data.to_vec().encode(s)
                        } ),+
                    }
                }
            }

            impl Decodable for $t {
                fn decode<D: Decoder>(d: &mut D) -> Result<$t, D::Error> {
                    let id: u8 = d.read_u8()?;
                    let v: Vec<u8> = d.read_to_vec(|d| d.read_u8())?;
                    match id {
                        $(
                            $idx => {
                                if v.len() != $x {
                                    return Err(d.error("Wrong array length"))
                                }
                                let mut a: [u8; $x] = [0; $x];
                                for i in 0..$x {
                                    a[i] = v[i];
                                }
                                Ok($t::$name(box a))
                            }
                        ),+

                        _ => Err(d.error("Unknown id")),
                    }
                }
            }
         )
}

generate_crypto!{
    PublicKey {
        ElGamal(256) = 0,
    }
}

impl Default for PublicKey {
    fn default() -> PublicKey {
        PublicKey::ElGamal(box [0; 256])
    }
}

generate_crypto!{
    PrivateKey {
        ElGamal(256) = 0,
    }
}

generate_crypto!{
    SessionKey {
        ElGamal(32) = 0,
    }
}

generate_crypto!{
    SigningPublicKey {
        DSA_SHA1(128) = 0,
        ECDSA_SHA256_P256(64) = 1,
        ECDSA_SHA384_P384(96) = 2,
        ECDSA_SHA512_P521(132) = 3,
        RSA_SHA256_2048(256) = 4,
        RSA_SHA384_3072(384) = 5,
        RSA_SHA512_4096(512) = 6,
        EdDSA_SHA512_Ed25519(32) = 7,
        EdDSA_SHA512_Ed25519ph(32) = 8,
    }
}

impl Default for SigningPublicKey {
    fn default() -> SigningPublicKey {
        SigningPublicKey::DSA_SHA1(box [0; 128])
    }
}

generate_crypto! {
    SigningPrivateKey {
        DSA_SHA1(20) = 0,
        ECDSA_SHA256_P256(32) = 1,
        ECDSA_SHA384_P384(48) = 2,
        ECDSA_SHA512_P521(66) = 3,
        RSA_SHA256_2048(512) = 4,
        RSA_SHA384_3072(768) = 5,
        RSA_SHA512_4096(1024) = 6,
        EdDSA_SHA512_Ed25519(32) = 7,
        EdDSA_SHA512_Ed25519ph(32) = 8,
    }
}

generate_crypto! {
    Signature {
        DSA_SHA1(40) = 0,
        ECDSA_SHA256_P256(64) = 1,
        ECDSA_SHA384_P384(96) = 2,
        ECDSA_SHA512_P521(132) = 3,
        RSA_SHA256_2048(256) = 4,
        RSA_SHA384_3072(384) = 5,
        RSA_SHA512_4096(512) = 6,
        EdDSA_SHA512_Ed25519(64) = 7,
        EdDSA_SHA512_Ed25519ph(64) = 8,
    }
}

impl Default for Signature {
    fn default() -> Signature {
        Signature::DSA_SHA1(box [0; 40])
    }
}

generate_crypto! {
    Hash {
        SHA256(32) = 0,
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum CertificateType {
    Null = 0,
    HashCash = 1,
    Hidden = 2,
    Signed = 3,
    Multiple = 4,
    Key = 5,
}

impl Default for CertificateType {
    fn default() -> CertificateType {
        CertificateType::Null
    }
}

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct Certificate {
    certificate_type: CertificateType,
    data: Vec<u8>,
}

#[derive(Debug, Default, RustcEncodable, RustcDecodable)]
pub struct KeysAndCert {
    public_key: PublicKey,
    signing_key: SigningPublicKey,
    certificate: Certificate,
}

pub type RouterIdentity = KeysAndCert;