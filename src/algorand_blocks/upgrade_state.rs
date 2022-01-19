use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_with::skip_serializing_none;

use crate::{algorand_errors::AlgorandError, algorand_types::Result};

#[skip_serializing_none]
#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpgradeState {
    #[serde(rename = "proto")]
    current_protocol: String,

    #[serde(rename = "nextproto")]
    next_protocol: Option<String>,

    #[serde(rename = "nextyes")]
    next_protocol_approvals: Option<u64>,

    #[serde(rename = "nextbefore")]
    next_protocol_vote_before: Option<u64>,

    #[serde(rename = "nextswitch")]
    next_protocol_switch_on: Option<u64>,
}

impl UpgradeState {
    pub fn from_json(json: &UpgradeStateJson) -> Self {
        Self {
            next_protocol: json.next_protocol.clone(),
            current_protocol: json.current_protocol.clone(),
            next_protocol_approvals: json.next_protocol_approvals,
            next_protocol_switch_on: json.next_protocol_switch_on,
            next_protocol_vote_before: json.next_protocol_vote_before,
        }
    }
}

impl FromStr for UpgradeState {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        UpgradeStateJson::from_str(s).map(|json| Self::from_json(&json))
    }
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpgradeStateJson {
    #[serde(rename = "current-protocol")]
    pub current_protocol: String,

    #[serde(rename = "next-protocol")]
    pub next_protocol: Option<String>,

    #[serde(rename = "next-protocol-approvals")]
    pub next_protocol_approvals: Option<u64>,

    #[serde(rename = "next-protocol-vote-before")]
    pub next_protocol_vote_before: Option<u64>,

    #[serde(rename = "next-protocol-switch-on")]
    pub next_protocol_switch_on: Option<u64>,
}

impl FromStr for UpgradeStateJson {
    type Err = AlgorandError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}
