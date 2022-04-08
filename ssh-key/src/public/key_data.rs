//! Public key data.

use super::Ed25519PublicKey;
use crate::{
    checked::CheckedSum, decode::Decode, encode::Encode, reader::Reader, writer::Writer, Algorithm,
    Error, Result,
};

#[cfg(feature = "alloc")]
use super::{DsaPublicKey, RsaPublicKey};

#[cfg(feature = "ecdsa")]
use {
    super::{ecdsa::EcdsaNistP256PublicKey, EcdsaPublicKey},
    crate::EcdsaCurve,
};

#[cfg(feature = "fingerprint")]
use crate::{Fingerprint, HashAlg, Sha256Fingerprint};

/// FIDO/U2F Security Key application string.
///
/// This is not presently customizable.
const SK_APPLICATION_STRING: &str = "ssh:";

/// Public key data.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum KeyData {
    /// Digital Signature Algorithm (DSA) public key data.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    Dsa(DsaPublicKey),

    /// Elliptic Curve Digital Signature Algorithm (ECDSA) public key data.
    #[cfg(feature = "ecdsa")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
    Ecdsa(EcdsaPublicKey),

    /// Ed25519 public key data.
    Ed25519(Ed25519PublicKey),

    /// RSA public key data.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    Rsa(RsaPublicKey),

    /// Security Key (FIDO/U2F) using ECDSA/NIST P-256 as specified in [PROTOCOL.u2f].
    ///
    /// [PROTOCOL.u2f]: https://cvsweb.openbsd.org/src/usr.bin/ssh/PROTOCOL.u2f?annotate=HEAD
    #[cfg(feature = "ecdsa")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
    SkEcdsaSha2NistP256(EcdsaNistP256PublicKey),

    /// Security Key (FIDO/U2F) using Ed25519 as specified in [PROTOCOL.u2f].
    ///
    /// [PROTOCOL.u2f]: https://cvsweb.openbsd.org/src/usr.bin/ssh/PROTOCOL.u2f?annotate=HEAD
    SkEd25519(Ed25519PublicKey),
}

impl KeyData {
    /// Get the [`Algorithm`] for this public key.
    pub fn algorithm(&self) -> Algorithm {
        match self {
            #[cfg(feature = "alloc")]
            Self::Dsa(_) => Algorithm::Dsa,
            #[cfg(feature = "ecdsa")]
            Self::Ecdsa(key) => key.algorithm(),
            Self::Ed25519(_) => Algorithm::Ed25519,
            #[cfg(feature = "alloc")]
            Self::Rsa(_) => Algorithm::Rsa { hash: None },
            #[cfg(feature = "ecdsa")]
            Self::SkEcdsaSha2NistP256(_) => Algorithm::SkEcdsaSha2NistP256,
            Self::SkEd25519(_) => Algorithm::SkEd25519,
        }
    }

    /// Get DSA public key if this key is the correct type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn dsa(&self) -> Option<&DsaPublicKey> {
        match self {
            Self::Dsa(key) => Some(key),
            _ => None,
        }
    }

    /// Get ECDSA public key if this key is the correct type.
    #[cfg(feature = "ecdsa")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
    pub fn ecdsa(&self) -> Option<&EcdsaPublicKey> {
        match self {
            Self::Ecdsa(key) => Some(key),
            _ => None,
        }
    }

    /// Get Ed25519 public key if this key is the correct type.
    pub fn ed25519(&self) -> Option<&Ed25519PublicKey> {
        match self {
            Self::Ed25519(key) => Some(key),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// Compute key fingerprint.
    ///
    /// Use [`Default::default()`] to use the default hash function (SHA-256).
    #[cfg(feature = "fingerprint")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fingerprint")))]
    pub fn fingerprint(&self, hash_alg: HashAlg) -> Result<Fingerprint> {
        match hash_alg {
            HashAlg::Sha256 => Sha256Fingerprint::try_from(self).map(Into::into),
            _ => Err(Error::Algorithm),
        }
    }

    /// Get RSA public key if this key is the correct type.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn rsa(&self) -> Option<&RsaPublicKey> {
        match self {
            Self::Rsa(key) => Some(key),
            _ => None,
        }
    }

    /// Get FIDO/U2F ECDSA/NIST P-256 public key if this key is the correct type.
    #[cfg(feature = "ecdsa")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
    pub fn sk_ecdsa_p256(&self) -> Option<&EcdsaNistP256PublicKey> {
        match self {
            Self::SkEcdsaSha2NistP256(key) => Some(key),
            _ => None,
        }
    }

    /// Get FIDO/U2F Ed25519 public key if this key is the correct type.
    pub fn sk_ed25519(&self) -> Option<&Ed25519PublicKey> {
        match self {
            Self::SkEd25519(key) => Some(key),
            _ => None,
        }
    }

    /// Is this key a DSA key?
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn is_dsa(&self) -> bool {
        matches!(self, Self::Dsa(_))
    }

    /// Is this key an ECDSA key?
    #[cfg(feature = "ecdsa")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
    pub fn is_ecdsa(&self) -> bool {
        matches!(self, Self::Ecdsa(_))
    }

    /// Is this key an Ed25519 key?
    pub fn is_ed25519(&self) -> bool {
        matches!(self, Self::Ed25519(_))
    }

    /// Is this key an RSA key?
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    pub fn is_rsa(&self) -> bool {
        matches!(self, Self::Rsa(_))
    }

    /// Is this key a FIDO/U2F ECDSA/NIST P-256 key?
    #[cfg(feature = "ecdsa")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
    pub fn is_sk_ecdsa_p256(&self) -> bool {
        matches!(self, Self::SkEcdsaSha2NistP256(_))
    }

    /// Is this key a FIDO/U2F Ed25519 key?
    pub fn is_sk_ed25519(&self) -> bool {
        matches!(self, Self::SkEd25519(_))
    }

    /// Decode [`KeyData`] for the specified algorithm.
    pub(crate) fn decode_as(reader: &mut impl Reader, algorithm: Algorithm) -> Result<Self> {
        match algorithm {
            #[cfg(feature = "alloc")]
            Algorithm::Dsa => DsaPublicKey::decode(reader).map(Self::Dsa),
            #[cfg(feature = "ecdsa")]
            Algorithm::Ecdsa { curve } => match EcdsaPublicKey::decode(reader)? {
                key if key.curve() == curve => Ok(Self::Ecdsa(key)),
                _ => Err(Error::Algorithm),
            },
            Algorithm::Ed25519 => Ed25519PublicKey::decode(reader).map(Self::Ed25519),
            #[cfg(feature = "alloc")]
            Algorithm::Rsa { .. } => RsaPublicKey::decode(reader).map(Self::Rsa),
            #[cfg(feature = "ecdsa")]
            Algorithm::SkEcdsaSha2NistP256 => {
                if EcdsaCurve::decode(reader)? != EcdsaCurve::NistP256 {
                    return Err(Error::Crypto);
                }

                let mut buf = [0u8; 65];
                let ec_point = EcdsaNistP256PublicKey::from_bytes(reader.read_byten(&mut buf)?)?;

                // application string (e.g. `ssh:`)
                // TODO(tarcieri): support for storing these?
                reader.drain_prefixed()?;

                Ok(Self::SkEcdsaSha2NistP256(ec_point))
            }
            Algorithm::SkEd25519 => {
                let public_key = Ed25519PublicKey::decode(reader)?;

                // application string (e.g. `ssh:`)
                // TODO(tarcieri): support for storing these?
                reader.drain_prefixed()?;

                Ok(Self::SkEd25519(public_key))
            }
            #[allow(unreachable_patterns)]
            _ => Err(Error::Algorithm),
        }
    }

    /// Get the encoded length of this key data without a leading algorithm
    /// identifier.
    pub(crate) fn encoded_key_data_len(&self) -> Result<usize> {
        match self {
            #[cfg(feature = "alloc")]
            Self::Dsa(key) => key.encoded_len(),
            #[cfg(feature = "ecdsa")]
            Self::Ecdsa(key) => key.encoded_len(),
            Self::Ed25519(key) => key.encoded_len(),
            #[cfg(feature = "alloc")]
            Self::Rsa(key) => key.encoded_len(),
            #[cfg(feature = "ecdsa")]
            Self::SkEcdsaSha2NistP256(key) => [
                EcdsaCurve::NistP256.encoded_len()?,
                key.as_bytes().encoded_len()?,
                SK_APPLICATION_STRING.encoded_len()?,
            ]
            .checked_sum(),
            Self::SkEd25519(key) => {
                [key.encoded_len()?, SK_APPLICATION_STRING.encoded_len()?].checked_sum()
            }
        }
    }

    /// Encode the key data without a leading algorithm identifier.
    pub(crate) fn encode_key_data(&self, writer: &mut impl Writer) -> Result<()> {
        match self {
            #[cfg(feature = "alloc")]
            Self::Dsa(key) => key.encode(writer),
            #[cfg(feature = "ecdsa")]
            Self::Ecdsa(key) => key.encode(writer),
            Self::Ed25519(key) => key.encode(writer),
            #[cfg(feature = "alloc")]
            Self::Rsa(key) => key.encode(writer),
            #[cfg(feature = "ecdsa")]
            Self::SkEcdsaSha2NistP256(key) => {
                EcdsaCurve::NistP256.encode(writer)?;
                key.as_bytes().encode(writer)?;
                SK_APPLICATION_STRING.encode(writer)
            }
            Self::SkEd25519(key) => {
                key.encode(writer)?;
                SK_APPLICATION_STRING.encode(writer)
            }
        }
    }
}

impl Decode for KeyData {
    fn decode(reader: &mut impl Reader) -> Result<Self> {
        let algorithm = Algorithm::decode(reader)?;
        Self::decode_as(reader, algorithm)
    }
}

impl Encode for KeyData {
    fn encoded_len(&self) -> Result<usize> {
        [
            self.algorithm().encoded_len()?,
            self.encoded_key_data_len()?,
        ]
        .checked_sum()
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<()> {
        self.algorithm().encode(writer)?;
        self.encode_key_data(writer)
    }
}