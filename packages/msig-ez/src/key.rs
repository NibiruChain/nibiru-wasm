use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pubkey {
    #[serde(rename = "@type")]
    pub key_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys: Option<Vec<Pubkey>>,
}

/// Custom error type for pubkey related operations
#[derive(Debug)]
pub enum PubkeyError {
    MissingKeyType,
    MissingKey,
    MissingThreshold,
    MissingPublicKeys,
    InvalidKeyType(String),
}

impl Pubkey {
    pub fn from_str_safe(pk_str: String) -> Pubkey {
        let clean_pk_str = pk_str.replace(r#"\""#, r#"""#);
        let err: String = format!(
            "failed to PubkeyRaw::from_string json parse: {}",
            clean_pk_str
        );
        serde_json::from_str(&clean_pk_str).unwrap_or_else(|_| panic!("{}", err))
    }

    pub fn to_pk_multi(&self) -> Result<PubkeyMultisig, PubkeyError> {
        // Check if key_type exists and is of the expected type
        let key_type = &self.key_type;
        if !key_type.contains("multisig") {
            return Err(PubkeyError::InvalidKeyType(key_type.clone()));
        }

        // Check if threshold exists
        let threshold = self.threshold.ok_or(PubkeyError::MissingThreshold)?;

        // Check if public_keys exists
        let public_keys: Vec<Pubkey> = self
            .public_keys
            .clone()
            .ok_or(PubkeyError::MissingPublicKeys)?;

        Ok(PubkeyMultisig {
            key_type: key_type.clone(),
            threshold,
            public_keys,
        })
    }

    /// True if the PubkeyRaw is a PubkeyMultiSig type
    pub fn is_multi(&self) -> bool {
        self.to_pk_multi().is_ok()
    }

    /// True if the PubkeyRaw is a PubkeySingleSig type
    pub fn is_single(&self) -> bool {
        self.to_pk_single().is_ok()
    }

    pub fn to_pk_single(&self) -> Result<PubkeySingleSig, PubkeyError> {
        // Check if key exists
        let key = self.key.clone().ok_or(PubkeyError::MissingKey)?;

        // Check if @type exists. This can be further refined if necessary.
        if self.key_type.is_empty() {
            return Err(PubkeyError::MissingKeyType);
        }

        Ok(PubkeySingleSig {
            key_type: self.key_type.clone(),
            key,
        })
    }

    pub fn as_cmd_arg(&self) -> String {
        serde_json::to_string(self)
            .expect("serde serizaliation to json string failed")
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Serialize the Pubkey to pretty-printed JSON
        let pk_json: String =
            serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        write!(formatter, "{}", pk_json)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PubkeyMultisig {
    #[serde(rename = "@type")]
    pub key_type: String,
    pub threshold: i32,
    pub public_keys: Vec<Pubkey>,
}

impl From<PubkeyMultisig> for Pubkey {
    fn from(pk: PubkeyMultisig) -> Self {
        Pubkey {
            key_type: pk.key_type,
            key: None, // Because Multisig doesn't have the `key` field
            threshold: Some(pk.threshold),
            public_keys: Some(pk.public_keys),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PubkeySingleSig {
    #[serde(rename = "@type")]
    pub key_type: String,
    pub key: String,
}

impl From<PubkeySingleSig> for Pubkey {
    fn from(pk: PubkeySingleSig) -> Self {
        Pubkey {
            key_type: pk.key_type,
            key: Some(pk.key),
            threshold: None, // Because SingleSig doesn't have the `threshold` field
            public_keys: None, // Because SingleSig doesn't have the `public_keys` field
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::key::Pubkey;

    pub const TEST_DATA_SINGLE: [&str; 2] = [
        r#" {\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A2HwcKQtNjk0kLizyJ4RNgHtgkjSJYNSPhZ8LNfYFbdK\"} "#,
        r#" {\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AjpZcCXRNGbjFBZ8HXlVgiKCfbrdaJ2SvRcBviUTG4JD\"} "#,
    ];

    pub const TEST_DATA_MULTI: [&str; 1] = [r#"
        {\"@type\":\"/cosmos.crypto.multisig.LegacyAminoPubKey\",\"threshold\":2,\"public_keys\":[{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A2HwcKQtNjk0kLizyJ4RNgHtgkjSJYNSPhZ8LNfYFbdK\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AxeuTLY+UELY+SOXFFl/guOy51gSwD1jRhOezs3qCIla\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AzGEhB9KTSbYGv1x57LRZqGo8x9ny+9wprHNkAn04z79\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A5FpaH7PvVhWkopoqMed+UZPoz5fp2qxgKJ+4G1XUpAl\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AjpZcCXRNGbjFBZ8HXlVgiKCfbrdaJ2SvRcBviUTG4JD\"}]} "#];

    pub const TEST_DATA_JQ: &str = r#"
{
  "@type": "/cosmos.crypto.multisig.LegacyAminoPubKey",
  "threshold": 2,
  "public_keys": [
    {
      "@type": "/cosmos.crypto.secp256k1.PubKey",
      "key": "A2HwcKQtNjk0kLizyJ4RNgHtgkjSJYNSPhZ8LNfYFbdK"
    },
    {
      "@type": "/cosmos.crypto.secp256k1.PubKey",
      "key": "AxeuTLY+UELY+SOXFFl/guOy51gSwD1jRhOezs3qCIla"
    },
    {
      "@type": "/cosmos.crypto.secp256k1.PubKey",
      "key": "AzGEhB9KTSbYGv1x57LRZqGo8x9ny+9wprHNkAn04z79"
    },
    {
      "@type": "/cosmos.crypto.secp256k1.PubKey",
      "key": "A5FpaH7PvVhWkopoqMed+UZPoz5fp2qxgKJ+4G1XUpAl"
    },
    {
      "@type": "/cosmos.crypto.secp256k1.PubKey",
      "key": "AjpZcCXRNGbjFBZ8HXlVgiKCfbrdaJ2SvRcBviUTG4JD"
    }
  ]
}
    "#;

    // fn parse_testdata() -> Vec<MsigSigner> {
    //     let signers: Vec<MsigSigner> =
    //         serde_json::from_str(TESTDATA).expect("failed to json parse testdata");
    //     assert_eq!(signers.len(), 3);
    //     return signers;
    // }

    fn parse_test_data(test_data: Vec<&str>) -> Vec<Pubkey> {
        return test_data
            .iter()
            .map(|testdata| -> Pubkey {
                Pubkey::from_str_safe(testdata.to_string())
            })
            .collect();
    }

    #[test]
    fn test_parse_single() {
        let signers: Vec<Pubkey> = parse_test_data(TEST_DATA_SINGLE.into());
        assert_eq!(signers.len(), 2);
        signers
            .iter()
            .for_each(|signer| assert!(signer.is_single()))
    }

    #[test]
    fn test_parse_multi() {
        let signers: Vec<Pubkey> = parse_test_data(TEST_DATA_MULTI.into());
        assert_eq!(signers.len(), 1);
        assert!(signers[0].is_multi())
    }

    #[test]
    fn test_parse_jq() {
        let signers: Vec<Pubkey> = parse_test_data(vec![TEST_DATA_JQ]);
        assert_eq!(signers.len(), 1);
        assert!(signers[0].is_multi())
    }
}
