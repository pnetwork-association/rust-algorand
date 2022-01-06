use std::fmt;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum AlgorandTransactionType {
    Pay,
    AssetTransfer,
    AssetConfiguration,
}

impl fmt::Display for AlgorandTransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pay => write!(f, "pay"),
            Self::AssetTransfer => write!(f, "axfer"),
            Self::AssetConfiguration => write!(f, "acfg"),
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
        serializer.serialize_str(&format!("{}", self))
    }
}
