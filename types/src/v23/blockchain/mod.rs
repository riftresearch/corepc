// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core `v23` - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

mod into;

use serde::{Deserialize, Serialize};

pub use super::{MempoolEntryError, MempoolEntryFees};

/// Result of JSON-RPC method `getmempoolentry`.
///
/// > getmempoolentry txid
/// >
/// > Returns mempool data for given transaction
/// >
/// > Arguments:
/// > 1. "txid"                 (string, required) The transaction id (must be in mempool)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetMempoolEntry(pub MempoolEntry);

/// A relative (ancestor or descendant) transaction of a transaction in the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MempoolEntry {
    /// Virtual transaction size as defined in BIP 141.
    ///
    /// This is different from actual serialized size for witness transactions as witness data is
    /// discounted. v0.19 and later only.
    pub vsize: i64,
    /// Transaction weight as defined in BIP 141.
    pub weight: i64,
    /// Transaction fee, denominated in BTC.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    pub fee: Option<f64>,
    /// Transaction fee with fee deltas used for mining priority, denominated in BTC.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    #[serde(rename = "modifiedfee")]
    pub modified_fee: Option<f64>,
    /// Local time transaction entered pool in seconds since 1 Jan 1970 GMT.
    pub time: i64,
    /// Block height when transaction entered pool.
    pub height: i64,
    /// Number of in-mempool descendant transactions (including this one).
    #[serde(rename = "descendantcount")]
    pub descendant_count: i64,
    /// Virtual transaction size of in-mempool descendants (including this one).
    #[serde(rename = "descendantsize")]
    pub descendant_size: i64,
    /// Transaction fees of in-mempool descendants (including this one) with fee deltas used for
    /// mining priority, denominated in sats.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    #[serde(rename = "descendantfees")]
    pub descendant_fees: Option<f64>,
    /// Number of in-mempool ancestor transactions (including this one).
    #[serde(rename = "ancestorcount")]
    pub ancestor_count: i64,
    /// Virtual transaction size of in-mempool ancestors (including this one).
    #[serde(rename = "ancestorsize")]
    pub ancestor_size: i64,
    /// Transaction fees of in-mempool ancestors (including this one) with fee deltas used for mining
    /// priority, denominated in sats.
    /// (DEPRECATED, returned only if config option -deprecatedrpc=fees is passed).
    #[serde(rename = "ancestorfees")]
    pub ancestor_fees: Option<f64>,
    /// Hash of serialized transaction, including witness data.
    pub wtxid: String,
    /// Fee object which contains the base fee, modified fee (with fee deltas), and ancestor/descendant fee totals all in BTC.
    pub fees: MempoolEntryFees,
    /// Unconfirmed transactions used as inputs for this transaction (parent transaction id).
    pub depends: Vec<String>,
    /// Unconfirmed transactions spending outputs from this transaction (child transaction id).
    #[serde(rename = "spentby")]
    pub spent_by: Vec<String>,
    /// Whether this transaction could be replaced due to BIP125 (replace-by-fee)
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by
    /// any peers)
    pub unbroadcast: bool,
}

/// Result of JSON-RPC method `savemempool`.
///
/// > savemempool
///
/// > Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SaveMempool {
    /// The directory and file where the mempool was saved.
    pub filename: String,
}
