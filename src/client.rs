// The per-action `ORDER.*` variants below are deprecated in favour of
// `Submit`. Allow `deprecated` within this module so serde's derived impls
// (which reference every variant) compile cleanly under `-D warnings`;
// downstream crates still receive the deprecation warning when they use them.
#![allow(deprecated)]

use serde::{Deserialize, Serialize};

use crate::RequestId;

/// Parameters for a transaction submission.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderParams {
    /// Base64-encoded raw transaction bytes
    pub tx: String,
}

/// Messages sent from client to server (Binance-style)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "method", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientMessage {
    /// Subscribe to market data topics or user data streams
    /// possible errors: ValidationError, TooManyRequests, InvalidSubscriptionFormat, Unauthorized,
    /// InvalidSymbol, ClientNotFound server responses: ServerMessageKind::Subscribe (success),
    /// ServerMessageKind::Error (failure)
    #[serde(alias = "subscribe")]
    Subscribe {
        id: Option<RequestId>,
        #[serde(default)]
        params: Vec<String>,
    },

    /// Unsubscribe from market data topics or user data streams
    /// possible errors: ValidationError (message parse)
    /// server responses: ServerMessageKind::Unsubscribe (success, idempotent - invalid topics
    /// silently skipped)
    #[serde(alias = "unsubscribe")]
    Unsubscribe {
        id: Option<RequestId>,
        #[serde(default)]
        params: Vec<String>,
    },

    /// List all active subscriptions for the client
    /// possible errors: ValidationError (message parse)
    /// server responses: ServerMessageKind::ListSubscriptions (success)
    #[serde(alias = "list_subscriptions")]
    ListSubscriptions { id: Option<RequestId> },

    /// Ping the server to check connection health
    /// possible errors: ValidationError (message parse)
    /// server responses: ServerMessageKind::Pong (success)
    #[serde(alias = "ping")]
    Ping { id: Option<RequestId> },

    /// Submit a signed transaction over the rollup WebSocket.
    ///
    /// The action — place, cancel, amend, cancel-all, cancel-and-place, etc. —
    /// is encoded in the signed transaction itself (`bullet-exchange-interface`'s
    /// `UserAction`); the server decodes the transaction and dispatches on the
    /// inner action. Prefer this over the per-action `ORDER.*` methods below,
    /// which restate — without enforcing — the action already in `tx`.
    #[serde(alias = "submit")]
    Submit { id: Option<RequestId>, params: OrderParams },

    /// Place an order via the rollup WebSocket.
    #[deprecated(
        since = "0.3.0",
        note = "use `ClientMessage::Submit`; the action is decoded from the signed tx"
    )]
    #[serde(alias = "order.place", rename = "ORDER.PLACE")]
    OrderPlace { id: Option<RequestId>, params: OrderParams },

    /// Cancel an order via the rollup WebSocket.
    #[deprecated(
        since = "0.3.0",
        note = "use `ClientMessage::Submit`; the action is decoded from the signed tx"
    )]
    #[serde(alias = "order.cancel", rename = "ORDER.CANCEL")]
    OrderCancel { id: Option<RequestId>, params: OrderParams },

    /// Amend an order via the rollup WebSocket.
    #[deprecated(
        since = "0.3.0",
        note = "use `ClientMessage::Submit`; the action is decoded from the signed tx"
    )]
    #[serde(alias = "order.amend", alias = "order.modify", rename = "ORDER.AMEND")]
    OrderAmend { id: Option<RequestId>, params: OrderParams },

    /// Cancel all open orders via the rollup WebSocket.
    #[deprecated(
        since = "0.3.0",
        note = "use `ClientMessage::Submit`; the action is decoded from the signed tx"
    )]
    #[serde(alias = "order.cancelAll", rename = "ORDER.CANCEL_ALL")]
    OrderCancelAll { id: Option<RequestId>, params: OrderParams },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_serializes_with_screaming_method() {
        let msg = ClientMessage::Submit {
            id: Some(RequestId::from(7u64)),
            params: OrderParams { tx: "deadbeef".to_string() },
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"method":"SUBMIT","id":7,"params":{"tx":"deadbeef"}}"#);
    }

    #[test]
    fn submit_accepts_lowercase_alias() {
        let json = r#"{"method":"submit","id":7,"params":{"tx":"deadbeef"}}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        assert!(matches!(msg, ClientMessage::Submit { .. }));
    }

    #[test]
    fn deprecated_order_place_still_round_trips() {
        // The legacy ORDER.* methods remain on the wire for backward compat
        // until the server and SDK migrate to SUBMIT.
        let json = r#"{"method":"ORDER.PLACE","id":1,"params":{"tx":"abc"}}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        assert!(matches!(msg, ClientMessage::OrderPlace { .. }));
    }
}
