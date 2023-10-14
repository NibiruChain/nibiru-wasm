mod traits;
mod type_url_nibiru;

pub use traits::*;
pub use type_url_nibiru::*;

pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("buf/cosmos.auth.v1beta1.rs");
        }
    }

    pub mod authz {
        pub mod v1beta1 {
            include!("buf/cosmos.authz.v1beta1.rs");
        }
    }

    pub mod bank {
        pub mod v1beta1 {
            include!("buf/cosmos.bank.v1beta1.rs");
        }
    }

    // TODO cosmos autocli

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!("buf/cosmos.base.abci.v1beta1.rs");
            }
        }

        /// Key-value pairs.
        pub mod kv {
            pub mod v1beta1 {
                include!("buf/cosmos.base.kv.v1beta1.rs");
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("buf/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Reflection support.
        pub mod reflection {
            pub mod v1beta1 {
                include!("buf/cosmos.base.reflection.v1beta1.rs");
            }

            pub mod v2alpha1 {
                include!("buf/cosmos.base.reflection.v2alpha1.rs");
            }
        }

        /// Snapshots containing Tendermint state sync info.
        pub mod snapshots {
            pub mod v1beta1 {
                include!("buf/cosmos.base.snapshots.v1beta1.rs");
            }
        }

        /// Data structure that holds the state of the application.
        pub mod store {
            pub mod v1beta1 {
                include!("buf/cosmos.base.store.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("buf/cosmos.base.v1beta1.rs");
        }

        pub mod tendermint {
            pub mod v1beta1 {
                include!("buf/cosmos.base.tendermint.v1beta1.rs");
            }
        }
    }
    // TODO cosmso base
    // TODO cosmos capability
    // TODO cosmos consensus
    // TODO cosmos crisis
    // TODO cosmos crypto
    // TODO cosmos distribution
    // TODO cosmos evidence
    // TODO cosmos feegrant
    // TODO cosmos genutil
    // TODO cosmos group
    // TODO cosmos mint
    // TODO cosmos nft
    // TODO cosmos orm
    // TODO cosmos params
    // TODO cosmos reflection
    // TODO cosmos slashing
    // TODO cosmos staking
    // TODO cosmos tx
    // TODO cosmos upgrade
    // TODO cosmos vesting
}

pub mod nibiru {
    pub mod devgas {
        include!("buf/nibiru.devgas.v1.rs");
    }
    pub mod epochs {
        include!("buf/nibiru.epochs.v1.rs");
    }
    pub mod genmsg {
        include!("buf/nibiru.genmsg.v1.rs");
    }
    pub mod inflation {
        include!("buf/nibiru.inflation.v1.rs");
    }
    pub mod oracle {
        include!("buf/nibiru.oracle.v1.rs");
    }
    pub mod perp {
        include!("buf/nibiru.perp.v2.rs");
    }
    pub mod spot {
        include!("buf/nibiru.spot.v1.rs");
    }
    pub mod sudo {
        include!("buf/nibiru.sudo.v1.rs");
    }
    pub mod tokenfactory {
        include!("buf/nibiru.tokenfactory.v1.rs");
    }
}

pub mod tendermint {
    pub mod abci {
        include!("buf/tendermint.abci.rs");
    }
    pub mod crypto {
        include!("buf/tendermint.crypto.rs");
    }
    pub mod p2p {
        include!("buf/tendermint.p2p.rs");
    }
    pub mod types {
        include!("buf/tendermint.types.rs");
    }
    pub mod version {
        include!("buf/tendermint.version.rs");
    }
}

#[cfg(test)]
mod tests {

    use super::{
        cosmos::{self, base::v1beta1::Coin},
        nibiru::perp,
    };

    #[test]
    fn nibiru_common_imports() {
        let _ = perp::MsgMarketOrder {
            sender: "sender".to_string(),
            pair: "nibi:usd".to_string(),
            side: perp::Direction::Long.into(),
            quote_asset_amount: "123".into(),
            leverage: "123".into(),
            base_asset_amount_limit: "0".into(),
        };
    }

    #[test]
    fn cosmos_imports() {
        let _ = cosmos::bank::v1beta1::MsgSend {
            from_address: "from".to_string(),
            to_address: "to".to_string(),
            amount: vec![Coin {
                denom: "unibi".to_string(),
                amount: "123".to_string(),
            }],
        };
    }
}
