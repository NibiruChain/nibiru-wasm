//! Implements the prost::Name trait for Nibiru protobuf types, which defines
//! the prost::Message.type_url function needed for CosmWasm smart contracts.

use prost::Name;

use crate::proto::nibiru;

const PACKAGE_TOKENFACTORY: &str = "nibiru.tokenfactory.v1";

impl Name for nibiru::tokenfactory::MsgCreateDenom {
    const NAME: &'static str = "MsgCreateDenom";
    const PACKAGE: &'static str = PACKAGE_TOKENFACTORY;
}

impl Name for nibiru::tokenfactory::MsgChangeAdmin {
    const NAME: &'static str = "MsgChangeAdmin";
    const PACKAGE: &'static str = PACKAGE_TOKENFACTORY;
}

impl Name for nibiru::tokenfactory::MsgUpdateModuleParams {
    const NAME: &'static str = "MsgUpdateModuleParams";
    const PACKAGE: &'static str = PACKAGE_TOKENFACTORY;
}

impl Name for nibiru::tokenfactory::MsgMint {
    const NAME: &'static str = "MsgMint";
    const PACKAGE: &'static str = PACKAGE_TOKENFACTORY;
}

impl Name for nibiru::tokenfactory::MsgBurn {
    const NAME: &'static str = "MsgBurn";
    const PACKAGE: &'static str = PACKAGE_TOKENFACTORY;
}

impl Name for nibiru::tokenfactory::MsgSetDenomMetadata {
    const NAME: &'static str = "MsgSetDenomMetadata";
    const PACKAGE: &'static str = PACKAGE_TOKENFACTORY;
}

#[cfg(test)]
mod tests {

    use crate::proto::{
        cosmos,
        nibiru::{self, tokenfactory::MsgMint},
        NibiruProstMsg, NibiruStargateMsg,
    };

    use cosmwasm_std as cw;

    #[test]
    fn stargate_tokenfactory() -> anyhow::Result<()> {
        let mut _pb: cw::CosmosMsg;
        let pb_msg: MsgMint = nibiru::tokenfactory::MsgMint::default();
        _pb = pb_msg.into_stargate_msg();
        if let cw::CosmosMsg::Stargate {
            type_url,
            value: _value,
        } = _pb
        {
            println!("full_name: {}", pb_msg.type_url());
            assert_eq!(type_url, "/nibiru.tokenfactory.v1.MsgMint")
        }
        _pb = nibiru::tokenfactory::MsgBurn::default().into_stargate_msg();
        _pb =
            nibiru::tokenfactory::MsgChangeAdmin::default().into_stargate_msg();
        _pb = nibiru::tokenfactory::MsgUpdateModuleParams::default()
            .into_stargate_msg();
        _pb = nibiru::tokenfactory::MsgSetDenomMetadata::default()
            .into_stargate_msg();

        Ok(())
    }

    /// Uses values produced from the chain's protobuf marshaler to verify that
    /// our `nibiru-std` types encode the same way.
    ///
    /// ```
    /// // For example, one test case came from:
    /// fmt.Printf("%v\n", encodingConfig.Marshaler.MustMarshal(&tokenfactorytypes.MsgCreateDenom{
    ///     Sender:   "sender",
    ///     Subdenom: "subdenom",
    /// }))
    /// // which outputs "[10 6 115 101 110 100 101 114 18 8 115 117 98 100 101 110 111 109]"
    /// ```
    #[test]
    fn stargate_encoding() -> anyhow::Result<()> {
        let test_cases: Vec<(Box<dyn NibiruProstMsg>, Vec<u8>)> = vec![
            (
                Box::new(nibiru::tokenfactory::MsgCreateDenom {
                            sender: String::from("sender"),
                            subdenom: String::from("subdenom"),
                        }),
                parse_byte_string(
                            "[10 6 115 101 110 100 101 114 18 8 115 117 98 100 101 110 111 109]",
                        ),
            ),
            (
                Box::new(nibiru::tokenfactory::MsgMint {
                    sender: String::from("sender"),
                    coin: Some(cosmos::base::v1beta1::Coin { denom: String::from("abcxyz"), amount: String::from("123") }),
                    mint_to: String::from("mint_to") }),
                vec![10u8, 6, 115, 101, 110, 100, 101, 114, 18, 13, 10, 6, 97, 98, 99, 120, 121, 122, 18, 3, 49, 50, 51, 26, 7, 109, 105, 110, 116, 95, 116, 111]
            ),
        ];

        for (pb_msg, want_bz) in &test_cases {
            println!("pb_msg {{ value: {:?} }}", pb_msg.to_bytes(),);
            assert_eq!(*want_bz, pb_msg.to_bytes(), "pb_msg: {pb_msg:?}");
        }

        Ok(())
    }

    fn parse_byte_string(s: &str) -> Vec<u8> {
        s.trim_start_matches('[')
            .trim_end_matches(']')
            .split_whitespace()
            .filter_map(|byte| byte.parse::<u8>().ok())
            .collect()
    }
}
