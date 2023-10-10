use serde::{Deserialize, Serialize};

use crate::key::Pubkey;

static KEY_TYPE_MULTI: &str = "multi";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsigSigner {
    pub name: String,
    #[serde(rename = "type")]
    pub key_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub pubkey: Pubkey,
}

pub fn parse_json_escapes(s: &str) -> String {
    s.replace(r#"\""#, r#"""#) // \" -> "
        .replace(r#""{""#, r#"{""#) // "{" -> {"
        .replace(r#"}""#, r#"}"#) // }" -> }
}

impl MsigSigner {
    pub fn gen_key_name(&self) -> String {
        format!("msmem-{}", self.name)
    }

    pub fn from_str(
        raw_str: &str,
    ) -> Result<Vec<MsigSigner>, serde_json::Error> {
        let clean_str = parse_json_escapes(raw_str);
        serde_json::from_str(&clean_str)
    }

    pub fn from_str_single(
        raw_str: &str,
    ) -> Result<MsigSigner, serde_json::Error> {
        let clean_str = raw_str.replace(r#"\""#, r#"""#);
        serde_json::from_str(&clean_str)
    }
}

#[cfg(test)]
mod tests {
    use super::MsigSigner;

    pub static TESTDATA: &str = r#"
    [
        {
            "name": "heisenberg",
            "type": "local",
            "address": "nibi1gc6vpl9j0ty8tkt53787zps9ezc70kj88hluw4",
            "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A2HwcKQtNjk0kLizyJ4RNgHtgkjSJYNSPhZ8LNfYFbdK\"}"
        },
        {
            "name": "matthias",
            "type": "offline",
            "address": "nibi16ukkende7k3q4lf7myqwmy8cds3peg3n2w69pv",
            "pubkey": "{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AjpZcCXRNGbjFBZ8HXlVgiKCfbrdaJ2SvRcBviUTG4JD\"}"
        },
        {
            "name": "nibimultisig",
            "type": "multi",
            "address": "nibi1qyqf35fkhn73hjr70442fctpq8prpqr9ysj9sn",
            "pubkey": "{\"@type\":\"/cosmos.crypto.multisig.LegacyAminoPubKey\",\"threshold\":2,\"public_keys\":[{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A2HwcKQtNjk0kLizyJ4RNgHtgkjSJYNSPhZ8LNfYFbdK\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AxeuTLY+UELY+SOXFFl/guOy51gSwD1jRhOezs3qCIla\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AzGEhB9KTSbYGv1x57LRZqGo8x9ny+9wprHNkAn04z79\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"A5FpaH7PvVhWkopoqMed+UZPoz5fp2qxgKJ+4G1XUpAl\"},{\"@type\":\"/cosmos.crypto.secp256k1.PubKey\",\"key\":\"AjpZcCXRNGbjFBZ8HXlVgiKCfbrdaJ2SvRcBviUTG4JD\"}]}"
        }
    ]
    "#;

    fn parse_testdata() -> Vec<MsigSigner> {
        let signers: Vec<MsigSigner> = MsigSigner::from_str(TESTDATA).unwrap();
        assert_eq!(signers.len(), 3);
        signers
    }

    #[test]
    fn test_parse_testdata() {
        parse_testdata();
    }

    #[test]
    fn test_deserialization() {
        let signers: Vec<MsigSigner> = parse_testdata();

        assert!(signers[0].pubkey.is_single());
        assert!(signers[1].pubkey.is_single());
        assert!(signers[2].pubkey.is_multi());

        // TODO
    }
}
