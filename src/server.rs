use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};

use crate::{RequestId, WSError};

/// client order id (u64 wrapper for type safety)
pub type ClientOrderId = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MessageType {
    #[serde(rename = "s")]
    Snapshot,
    #[serde(rename = "u")]
    Update,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataMessage {
    pub channel: String,
    pub symbol: String,
    pub ts: u64,
    #[serde(rename = "mt")]
    pub msg_type: MessageType,
    pub data: serde_json::Value,
}

/// Status message for connection lifecycle events
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    /// Event time (ms)
    #[serde(rename = "E")]
    pub event_time: u64,
    pub status: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PongMessage {
    pub id: Option<RequestId>,
    /// Event time (ms)
    #[serde(rename = "E")]
    pub event_time: u64,
}

/// Error message from the server
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
    /// Event time (ms)
    #[serde(rename = "E")]
    pub event_time: u64,
    pub error: WSError,
}

/// Price level as [price, quantity]
#[derive(Clone, Debug)]
pub struct PriceLevel(pub String, pub String);

impl Serialize for PriceLevel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.0)?;
        tuple.serialize_element(&self.1)?;
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for PriceLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (price, qty) = <(String, String)>::deserialize(deserializer)?;
        Ok(PriceLevel(price, qty))
    }
}

/// Binance-compatible depth update message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DepthUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub last_update_id: u64,
    #[serde(rename = "pu")]
    pub prev_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<PriceLevel>,
    #[serde(rename = "a")]
    pub asks: Vec<PriceLevel>,
    #[serde(rename = "mt")]
    pub msg_type: MessageType,
}

/// AggTrade message with DEX-specific fields
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AggTradeMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    // DEX-specific fields
    #[serde(rename = "th")]
    pub tx_hash: String,
    #[serde(rename = "ua")]
    pub user_address: String,
    #[serde(rename = "oi")]
    pub order_id: u64,
    #[serde(rename = "mk")]
    pub is_maker: bool,
    #[serde(rename = "ff")]
    pub is_full_fill: bool,
    #[serde(rename = "lq")]
    pub is_liquidation: bool,
    #[serde(rename = "fe")]
    pub fee: String,
    #[serde(rename = "nf")]
    pub net_fee: String,
    #[serde(rename = "fa")]
    pub fee_asset: String,
    #[serde(rename = "co", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
    #[serde(rename = "sd")]
    pub side: String,
}

/// Binance-compatible bookTicker (BBO) message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookTickerMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "u")]
    pub update_id: u64,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub best_bid_price: String,
    #[serde(rename = "B")]
    pub best_bid_qty: String,
    #[serde(rename = "a")]
    pub best_ask_price: String,
    #[serde(rename = "A")]
    pub best_ask_qty: String,
    #[serde(rename = "mt")]
    pub msg_type: MessageType,
}

/// Binance-compatible forceOrder message for liquidation trades
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ForceOrderMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "o")]
    pub order: ForceOrderDetails,
}

/// Order details within a forceOrder message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ForceOrderDetails {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
    pub filled_qty: Option<String>,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "ap")]
    pub avg_price: String,
    #[serde(rename = "X")]
    pub status: String,
    #[serde(rename = "l")]
    pub last_filled_qty: String,
    #[serde(rename = "T")]
    pub trade_time: u64,
    // DEX-specific fields
    #[serde(rename = "th")]
    pub tx_hash: String,
    #[serde(rename = "ua")]
    pub user_address: String,
    #[serde(rename = "oi")]
    pub order_id: u64,
    #[serde(rename = "ti")]
    pub trade_id: u64,
}

/// Binance-compatible markPrice message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MarkPriceMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub mark_price: String,
    #[serde(rename = "i")]
    pub index_price: String,
    #[serde(rename = "P", skip_serializing_if = "Option::is_none")]
    pub estimated_settle_price: Option<String>,
    #[serde(rename = "r")]
    pub funding_rate: String,
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub next_funding_time: Option<u64>,
    #[serde(rename = "th", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

/// User order update message (Binance ORDER_TRADE_UPDATE style)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderUpdateMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "o")]
    pub order: OrderUpdateData,
}

/// Common fields for all order update events
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderUpdateCommon {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "co", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
    #[serde(rename = "X")]
    pub status: String,
    #[serde(rename = "x")]
    pub execution_type: String,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "th")]
    pub tx_hash: String,
    #[serde(rename = "ua")]
    pub user_address: String,
}

/// Order data for NEW order placement
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlaceOrderData {
    #[serde(flatten)]
    pub common: OrderUpdateCommon,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "o")]
    pub order_type: String,
    #[serde(rename = "f")]
    pub time_in_force: String,
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
}

/// Order data for CANCELED orders
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CancelOrderData {
    #[serde(flatten)]
    pub common: OrderUpdateCommon,
}

/// Order data for TRADE fills
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TradeFillData {
    #[serde(flatten)]
    pub common: OrderUpdateCommon,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "l")]
    pub last_filled_qty: String,
    #[serde(rename = "L")]
    pub last_filled_price: String,
    #[serde(rename = "n")]
    pub commission: String,
}

/// Untagged enum - serializes directly as the variant's fields
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum OrderUpdateData {
    TradeFill(TradeFillData),
    PlaceOrder(PlaceOrderData),
    Cancel(CancelOrderData),
}
