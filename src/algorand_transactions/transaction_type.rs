use std::{default::Default, fmt, str::FromStr};

use serde::{Deserialize, Serialize, Serializer};

use crate::{algorand_errors::AlgorandError, algorand_types::Result};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum AlgorandTransactionType {
    Pay,
    StateProof,
    AssetFreeze,
    Unrecognized,
    AssetTransfer,
    ApplicationCall,
    KeyRegistration,
    AssetConfiguration,
}

impl fmt::Display for AlgorandTransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pay => write!(f, "pay"),
            Self::StateProof => write!(f, "stpf"),
            Self::AssetFreeze => write!(f, "afrz"),
            Self::AssetTransfer => write!(f, "axfer"),
            Self::ApplicationCall => write!(f, "appl"),
            Self::KeyRegistration => write!(f, "keyreg"),
            Self::AssetConfiguration => write!(f, "acfg"),
            Self::Unrecognized => write!(f, "unrecognized"),
        }
    }
}

impl Serialize for AlgorandTransactionType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl Default for AlgorandTransactionType {
    fn default() -> Self {
        Self::Pay
    }
}

impl FromStr for AlgorandTransactionType {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "pay" => Ok(Self::Pay),
            "stpf" => Ok(Self::StateProof),
            "afrz" => Ok(Self::AssetFreeze),
            "axfer" => Ok(Self::AssetTransfer),
            "appl" => Ok(Self::ApplicationCall),
            "keyreg" => Ok(Self::KeyRegistration),
            "acfg" => Ok(Self::AssetConfiguration),
            _ => Ok(Self::Unrecognized),
        }
    }
}
