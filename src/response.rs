//! Server response types for order RPC acks (`order.place`, `order.cancel`,
//! `order.amend`, `order.cancelAll`).

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
    pub id: Option<RequestId>,
    /// Event time (µs).
    #[serde(rename = "E")]
    pub event_time: u64,
    pub results: OrderResultPayload,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_full_payload() {
        let json = r#"{
            "id": 42,
            "E": 1706745600000000,
            "results": {
                "tx_id": "0xabc123",
                "status": "processed",
                "order_ids": [1, 2],
                "client_order_ids": [100, 101]
            }
        }"#;
        let msg: OrderResultMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.id, Some(RequestId::from(42u64)));
        assert_eq!(msg.event_time, 1_706_745_600_000_000);
        assert_eq!(msg.results.tx_id, "0xabc123");
        assert_eq!(msg.results.status, TxStatus::Processed);
        assert_eq!(msg.results.order_ids, vec![1, 2]);
        assert_eq!(msg.results.client_order_ids, vec![100, 101]);
    }

    #[test]
    fn id_absent_defaults_to_none() {
        // Server may omit `id` for unsolicited pushes.
        let json = r#"{"E":1706745600000000,"results":{"tx_id":"0x1","status":"submitted"}}"#;
        let msg: OrderResultMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.id, None);
    }

    #[test]
    fn optional_vec_fields_absent() {
        // cancelAll acks commonly omit client_order_ids.
        let json =
            r#"{"id":1,"E":1706745600000000,"results":{"tx_id":"0x2","status":"processed","order_ids":[5]}}"#;
        let msg: OrderResultMessage = serde_json::from_str(json).unwrap();
        assert!(msg.results.client_order_ids.is_empty());
    }

    #[test]
    fn tx_status_is_success() {
        assert!(TxStatus::Processed.is_success());
        assert!(TxStatus::Published.is_success());
        assert!(TxStatus::Submitted.is_success());
        assert!(TxStatus::Finalized.is_success());
        assert!(!TxStatus::Dropped.is_success());
        assert!(!TxStatus::Unknown.is_success());
    }
}
