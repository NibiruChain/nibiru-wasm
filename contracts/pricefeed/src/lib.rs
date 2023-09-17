use crate::error::ContractError;

pub mod contract;
pub mod error;
pub mod events;
pub mod msgs;
pub mod state;

const PAIR_SEPARATOR: char = ':';

pub struct AssetPair {
    pub token0: String,
    pub token1: String,
}

impl TryFrom<String> for AssetPair {
    type Error = ContractError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let res: Vec<&str> = value.split(PAIR_SEPARATOR).collect();
        if res.len() != 2 {
            return Err(Self::Error::InvalidAssetPair(
                "invalid separator".to_string(),
            ));
        }

        // maybe regexp...
        Ok(Self {
            token0: res[0].to_string(),
            token1: res[1].to_string(),
        })
    }
}

impl AssetPair {
    pub fn new(token0: String, token1: String) -> Self {
        Self { token0, token1 }
    }
    pub fn inverse(&self) -> Self {
        Self {
            token0: self.token0.clone(),
            token1: self.token1.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        // not clean but whatever... speeding
        return [self.token0.clone(), self.token1.clone()]
            .join(PAIR_SEPARATOR.to_string().as_str());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
