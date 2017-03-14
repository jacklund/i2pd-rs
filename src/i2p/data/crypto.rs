use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

pub enum PublicKey {
    ElGamal([u8; 256]),
}

pub enum PrivateKey {
    ElGamal([u8; 256]),
}

pub enum SessionKey {
    ElGamal([u8; 32]),
}

pub enum SigningPublicKey {
    DSA_SHA1([u8; 128]),
    ECDSA_SHA256_P256([u8; 64]),
    ECDSA_SHA384_P384([u8; 96]),
    ECDSA_SHA512_P521([u8; 132]),
    RSA_SHA256_2048([u8; 256]),
    RSA_SHA384_3072([u8; 384]),
    RSA_SHA512_4096([u8; 512]),
    EdDSA_SHA512_Ed25519([u8; 32]),
    EdDSA_SHA512_Ed25519ph([u8; 32]),
}

pub enum SigningPrivateKey {
    DSA_SHA1([u8; 20]),
    ECDSA_SHA256_P256([u8; 32]),
    ECDSA_SHA384_P384([u8; 48]),
    ECDSA_SHA512_P521([u8; 66]),
    RSA_SHA256_2048([u8; 512]),
    RSA_SHA384_3072([u8; 768]),
    RSA_SHA512_4096([u8; 1024]),
    EdDSA_SHA512_Ed25519([u8; 32]),
    EdDSA_SHA512_Ed25519ph([u8; 32]),
}

pub enum Signature {
    DSA_SHA1([u8; 40]),
    ECDSA_SHA256_P256([u8; 64]),
    ECDSA_SHA384_P384([u8; 96]),
    ECDSA_SHA512_P521([u8; 132]),
    RSA_SHA256_2048([u8; 256]),
    RSA_SHA384_3072([u8; 384]),
    RSA_SHA512_4096([u8; 512]),
    EdDSA_SHA512_Ed25519([u8; 64]),
    EdDSA_SHA512_Ed25519ph([u8; 64]),
}

pub enum Hash {
    SHA256([u8; 32]),
}

pub enum CertificateType {
    Null = 0,
    HashCash = 1,
    Hidden = 2,
    Signed = 3,
    Multiple = 4,
    Key = 5,
}

pub struct Certificate {
    certificate_type: CertificateType,
    data: Vec<u8>,
}

#[derive(Debug)]
pub struct KeysAndCert {
    public_key: PublicKey,
    signing_key: SigningPublicKey,
    certificate: Certificate,
}

pub type RouterIdentity = KeysAndCert;