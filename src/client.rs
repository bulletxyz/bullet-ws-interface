use serde::{Deserialize, Serialize};

use crate::RequestId;

/// Parameters for order operations
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderParams {
    /// Base64-encoded raw transaction bytes
    pub tx: String,
}

/// Messages sent from client to server (Binance-style)
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "method", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientMessage {
    #[serde(alias = "subscribe")]
    Subscribe {
        id: Option<RequestId>,
        #[serde(default)]
        params: Vec<String>,
    },

    #[serde(alias = "unsubscribe")]
    Unsubscribe {
        id: Option<RequestId>,
        #[serde(default)]
        params: Vec<String>,
    },

    #[serde(alias = "list_subscriptions")]
    ListSubscriptions { id: Option<RequestId> },

    #[serde(alias = "ping")]
    Ping { id: Option<RequestId> },

    /// Place an order via the rollup WebSocket
    #[serde(alias = "order.place", rename = "ORDER.PLACE")]
    OrderPlace {
        id: Option<RequestId>,
        params: OrderParams,
    },

    /// Cancel an order via the rollup WebSocket
    #[serde(alias = "order.cancel", rename = "ORDER.CANCEL")]
    OrderCancel {
        id: Option<RequestId>,
        params: OrderParams,
    },

    /// Amend an order via the rollup WebSocket
    #[serde(alias = "order.amend", alias = "order.modify", rename = "ORDER.AMEND")]
    OrderAmend {
        id: Option<RequestId>,
        params: OrderParams,
    },

    /// Cancel all open orders via the rollup WebSocket
    #[serde(alias = "order.cancelAll", rename = "ORDER.CANCEL_ALL")]
    OrderCancelAll {
        id: Option<RequestId>,
        params: OrderParams,
    },
}
