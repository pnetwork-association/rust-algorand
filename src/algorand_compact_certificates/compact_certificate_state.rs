use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    algorand_hash::AlgorandHash,
    algorand_micro_algos::MicroAlgos,
    algorand_types::Result,
    errors::AppError,
};

/// Tracks the state of compact certificates.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompactCertificateState {
    /// The root of a Merkle tree containing the online accounts that will help
    /// sign a compact certificate. The Merkle root, and the compact certificate,
    /// happen on blocks that are a multiple of ConsensusParams.CompactCertRounds.
    /// For blocks that are not a multiple of ConsensusParams.CompactCertRounds, this value is
    /// zero.
    #[serde(rename = "v")]
    compact_cert_voters: AlgorandHash,

    /// The total number of MicroAlgos held by the accounts in `compact_cert_voters`
    /// (or zero, if the merkle root is zero). This is intended for computing the threshold
    /// of votes to expect from `compact_cert_voters`.
    #[serde(rename = "t")]
    compact_cert_voters_total: MicroAlgos,

    /// The next round for which we will accept a CompactCert transaction.
    #[serde(rename = "n")]
    compact_cert_next_round: u64,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CompactCertificateStateJson {
    #[serde(rename = "compact-cert-voters")]
    pub compact_cert_voters: String,

    #[serde(rename = "compact-cert-next-round")]
    pub compact_cert_next_round: u64,

    #[serde(rename = "compact-cert-voters-total")]
    pub compact_cert_voters_total: u64,
}

impl FromStr for CompactCertificateStateJson {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl FromStr for CompactCertificateState {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        CompactCertificateStateJson::from_str(s).and_then(|json| Self::from_json(&json))
    }
}

impl CompactCertificateState {
    pub fn from_json(json: &CompactCertificateStateJson) -> Result<Self> {
        Ok(Self {
            compact_cert_next_round: json.compact_cert_next_round,
            compact_cert_voters_total: MicroAlgos(json.compact_cert_voters_total),
            compact_cert_voters: AlgorandHash::from_str(&json.compact_cert_voters)?,
        })
    }

    fn to_json(&self) -> CompactCertificateStateJson {
        CompactCertificateStateJson {
            compact_cert_next_round: self.compact_cert_next_round,
            compact_cert_voters: self.compact_cert_voters.to_string(),
            compact_cert_voters_total: self.compact_cert_voters_total.0,
        }
    }
}
