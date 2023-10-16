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
        nibiru::{self, tokenfactory::MsgMint},
        NibiruStargateMsg,
    };

    use cosmwasm_std as cw;

    #[test]
    fn stargate_tokenfactory() -> anyhow::Result<()> {
        let mut _pb: cw::CosmosMsg;
        let pb_msg: MsgMint = nibiru::tokenfactory::MsgMint::default();
        _pb = pb_msg.into_stargate_msg()?;
        if let cw::CosmosMsg::Stargate {
            type_url,
            value: _value,
        } = _pb
        {
            println!("full_name: {}", pb_msg.type_url());
            assert_eq!(type_url, "/nibiru.tokenfactory.v1.MsgMint")
        }
        _pb = nibiru::tokenfactory::MsgBurn::default().into_stargate_msg()?;
        _pb = nibiru::tokenfactory::MsgChangeAdmin::default()
            .into_stargate_msg()?;
        _pb = nibiru::tokenfactory::MsgUpdateModuleParams::default()
            .into_stargate_msg()?;
        _pb = nibiru::tokenfactory::MsgSetDenomMetadata::default()
            .into_stargate_msg()?;

        Ok(())
    }
}
