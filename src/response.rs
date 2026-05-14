use serde::{Deserialize, Serialize};

use crate::RequestId;

/// Transaction status returned by the sequencer in an order RPC ack.
/// See <https://tradingapi.bullet.xyz/docs/ws/index.html#request-response>.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TxStatus {
    /// Executed by the sequencer — `order_ids` is populated.
    Processed,
    /// Accepted, will be processed in a subsequent block.
    Published,
    /// Received by the sequencer but not yet published.
    Submitted,
    /// Finalized on-chain.
    Finalized,
    /// Dropped (expired uniqueness, duplicate generation value).
    Dropped,
    /// Status could not be determined.
    Unknown,
}

impl TxStatus {
    /// Returns `true` for statuses that mean the order is (or will be) live on
    /// the book. `Dropped` and `Unknown` are non-success — the caller should
    /// treat the operation as failed and reconcile.
    #[must_use]
    pub fn is_success(self) -> bool {
        matches!(self, Self::Processed | Self::Published | Self::Submitted | Self::Finalized)
    }
}

/// Inner `results` payload for a successful order RPC ack.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderResultPayload {
    /// Transaction hash.
    pub tx_id: String,
    /// Sequencer status — see [`TxStatus`].
    pub status: TxStatus,
    /// Order IDs affected by this transaction. Populated when status is
    /// `processed`; may be empty otherwise.
    #[serde(default)]
    pub order_ids: Vec<u64>,
    /// Client order IDs corresponding to `order_ids`, in matching positions.
    #[serde(default)]
    pub client_order_ids: Vec<u64>,
}

/// Result message for `order.place` / `order.cancel` / `order.amend` /
/// `order.cancelAll` RPCs. Correlate to the originating request via [`id`].
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderResultMessage {
    #[serde(default)]
    pub id: Option<RequestId>,
    /// Event time (µs).
    #[serde(rename = "E")]
    pub event_time: u64,
    pub results: OrderResultPayload,
}
