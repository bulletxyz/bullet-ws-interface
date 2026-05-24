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

/// Generic success envelope returned for `subscribe`, `unsubscribe`, etc.
/// Wraps a `result: "success"` value plus the request id (echoed back from
/// the client message) and the server-side event time.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MethodResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
    /// Event time (us)
    #[serde(rename = "E")]
    pub event_time: u64,
    /// Always `"success"` for these envelopes.
    pub result: String,
}

/// Response to a `subscribe` client request.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct SubscribeOk(pub MethodResult);

/// Response to an `unsubscribe` client request.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct UnsubscribeOk(pub MethodResult);

/// Response to a `list_subscriptions` client request.
///
/// `result` is the list of currently-subscribed topics for the connection.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListSubscriptionsMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
    /// Event time (us)
    #[serde(rename = "E")]
    pub event_time: u64,
    pub result: Vec<String>,
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

/// AggTrade message with DEX-specific fields.
///
/// As of trading-api 2026-05-23, the DEX-specific fields (`ua`, `oi`, `mk`,
/// `ff`, `lq`, `fe`, `nf`, `fa`, `co`, `sd`, `ft`, `z`, `Z`, `rs`) are
/// emitted as deprecated empty/zero placeholders. Consumers should treat
/// their values as meaningless and use `@user.orders` for real fill data.
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
    // DEX-specific fields — all deprecated as of 2026-05-23; use @user.orders.
    #[serde(rename = "th")]
    pub tx_hash: String,
    /// Deprecated: empty-string placeholder as of 2026-05-23. Use `@user.orders`.
    #[serde(rename = "ua", default, skip_serializing_if = "Option::is_none")]
    pub user_address: Option<String>,
    /// Deprecated: zero placeholder as of 2026-05-23. Use `@user.orders` `i` field.
    #[serde(rename = "oi", default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
    /// Deprecated: empty placeholder as of 2026-05-23. Use `@user.orders` `m` field.
    #[serde(rename = "mk", default, skip_serializing_if = "Option::is_none")]
    pub is_maker: Option<bool>,
    /// Deprecated: empty placeholder as of 2026-05-23. Use order status `X` or `rs == 0`.
    #[serde(rename = "ff", default, skip_serializing_if = "Option::is_none")]
    pub is_full_fill: Option<bool>,
    /// Deprecated: empty placeholder as of 2026-05-23. Use `@user.orders` fill type `ft`.
    #[serde(rename = "lq", default, skip_serializing_if = "Option::is_none")]
    pub is_liquidation: Option<bool>,
    /// Deprecated: zero placeholder as of 2026-05-23. Use `@user.orders` `n`.
    #[serde(rename = "fe", default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Deprecated: zero placeholder as of 2026-05-23.
    #[serde(rename = "nf", default, skip_serializing_if = "Option::is_none")]
    pub net_fee: Option<String>,
    /// Deprecated: empty-string placeholder as of 2026-05-23. Use `@user.orders` `N`.
    #[serde(rename = "fa", default, skip_serializing_if = "Option::is_none")]
    pub fee_asset: Option<String>,
    /// Deprecated: empty placeholder as of 2026-05-23. Use `@user.orders` `co`.
    #[serde(rename = "co", default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
    /// Deprecated: empty-string placeholder as of 2026-05-23. Use `@user.orders` `S`.
    #[serde(rename = "sd", default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Deprecated: emitted as zero/empty placeholder. Use `@user.orders` `ft`.
    #[serde(rename = "ft", default, skip_serializing_if = "Option::is_none")]
    pub fill_type: Option<String>,
    /// Deprecated: emitted as zero/empty placeholder. Use `@user.orders` `z`.
    #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_filled_size: Option<String>,
    /// Deprecated: emitted as zero/empty placeholder. Use `@user.orders` `Z`.
    #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_filled_cot: Option<String>,
    /// Deprecated: emitted as zero/empty placeholder. Use `@user.orders` `rs`.
    #[serde(rename = "rs", default, skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<String>,
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
    /// Deprecated as of 2026-05-23: empty-string placeholder. The address is
    /// implicit from the authenticated user stream, so the field carries no
    /// information. Will be dropped from the wire in a future release.
    #[serde(rename = "ua", default, skip_serializing_if = "Option::is_none")]
    pub user_address: Option<String>,
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

/// Order data for TRADE fills.
///
/// Fields populated only for orderbook fills (when the rollup emits cumulative
/// counters) are wrapped in `Option`: `ap`, `ft`, `z`, `Z`, `rs`. Liquidation
/// fills typically omit these — use `X == "FILLED"` from `OrderUpdateCommon`
/// as the canonical "fully filled" signal in that case.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TradeFillData {
    #[serde(flatten)]
    pub common: OrderUpdateCommon,
    #[serde(rename = "S")]
    pub side: String,
    #[serde(rename = "p", default, skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Average fill price across cumulative fills. Added 2026-05-23.
    #[serde(rename = "ap", default, skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<String>,
    #[serde(rename = "q", default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(rename = "l")]
    pub last_filled_qty: String,
    #[serde(rename = "L")]
    pub last_filled_price: String,
    #[serde(rename = "n")]
    pub commission: String,
    /// Commission asset (e.g. `"USDC"`). Added 2026-05-23.
    #[serde(rename = "N")]
    pub commission_asset: String,
    /// Whether this fill was on the maker side.
    #[serde(rename = "m")]
    pub is_maker: bool,
    /// Sequencer-assigned trade id.
    #[serde(rename = "t")]
    pub trade_id: u64,
    /// Realized PnL for this fill. Added 2026-05-23.
    #[serde(rename = "rp")]
    pub realized_pnl: String,
    /// Fill type: `"orderbook"` (`"o"`) for book fills, `"liquidation"` (`"l"`)
    /// for liquidation fills. Absent on legacy fills.
    #[serde(rename = "ft", default, skip_serializing_if = "Option::is_none")]
    pub fill_type: Option<String>,
    /// Cumulative filled size across partial fills. Absent when upstream did
    /// not provide it (e.g. liquidations).
    #[serde(rename = "z", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_filled_size: Option<String>,
    /// Cumulative quote notional filled. Absent when upstream did not provide it.
    #[serde(rename = "Z", default, skip_serializing_if = "Option::is_none")]
    pub cumulative_filled_cot: Option<String>,
    /// Remaining size on the order (`"0"` means fully filled). Absent when
    /// upstream did not provide it; fall back to `X == "FILLED"` for that case.
    #[serde(rename = "rs", default, skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<String>,
}

/// Untagged enum - serializes directly as the variant's fields
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum OrderUpdateData {
    /// Boxed because `TradeFillData` is much larger than the other variants
    /// (clippy::large_enum_variant); matches trading-api's wrapping.
    TradeFill(Box<TradeFillData>),
    PlaceOrder(PlaceOrderData),
    Cancel(CancelOrderData),
}

#[cfg(test)]
mod wire_format_tests {
    //! Regression tests pinning each server message type to the actual JSON
    //! shape captured live from `tradingapi.testnet.bullet.xyz` (2026-05-24).
    //! If trading-api changes its wire format, these break here first.

    use super::*;

    #[test]
    fn subscribe_ok_round_trips_wire_payload() {
        let wire = r#"{"e":"subscribe","id":1,"E":1779600272876932,"result":"success"}"#;
        let parsed: MethodResult = serde_json::from_str(wire).expect("MethodResult deserializes");
        assert_eq!(parsed.id, Some(RequestId::from(1)));
        assert_eq!(parsed.event_time, 1779600272876932);
        assert_eq!(parsed.result, "success");
    }

    #[test]
    fn list_subscriptions_round_trips_wire_payload() {
        let wire = r#"{"e":"list_subscriptions","id":2,"E":1779600273218722,"result":["ETH-USD@aggTrade","BTC-USD@bookTicker"]}"#;
        let parsed: ListSubscriptionsMessage =
            serde_json::from_str(wire).expect("ListSubscriptionsMessage deserializes");
        assert_eq!(parsed.id, Some(RequestId::from(2)));
        assert_eq!(parsed.event_time, 1779600273218722);
        assert_eq!(
            parsed.result,
            vec!["ETH-USD@aggTrade".to_string(), "BTC-USD@bookTicker".to_string()]
        );
    }

    #[test]
    fn trade_fill_round_trips_wire_payload() {
        let wire = r#"{"s":"SOL-USD","i":183696108,"X":"FILLED","x":"TRADE","T":1779598581565646,"th":"0x44dd","ua":"","S":"BUY","ap":"85.96","l":"0.1","L":"85.96","n":"0.00275072","N":"USDC","m":false,"t":13196983,"rp":"-0.0156","ft":"o","z":"0.1","Z":"8.596","rs":"0"}"#;
        let parsed: TradeFillData = serde_json::from_str(wire).expect("TradeFillData deserializes");
        assert_eq!(parsed.common.symbol, "SOL-USD");
        assert_eq!(parsed.common.order_id, 183696108);
        assert_eq!(parsed.common.status, "FILLED");
        assert_eq!(parsed.common.execution_type, "TRADE");
        assert_eq!(parsed.common.user_address.as_deref(), Some(""));
        assert_eq!(parsed.side, "BUY");
        assert_eq!(parsed.avg_price.as_deref(), Some("85.96"));
        assert_eq!(parsed.last_filled_qty, "0.1");
        assert_eq!(parsed.last_filled_price, "85.96");
        assert_eq!(parsed.commission, "0.00275072");
        assert_eq!(parsed.commission_asset, "USDC");
        assert!(!parsed.is_maker);
        assert_eq!(parsed.trade_id, 13196983);
        assert_eq!(parsed.realized_pnl, "-0.0156");
        assert_eq!(parsed.fill_type.as_deref(), Some("o"));
        assert_eq!(parsed.cumulative_filled_size.as_deref(), Some("0.1"));
        assert_eq!(parsed.cumulative_filled_cot.as_deref(), Some("8.596"));
        assert_eq!(parsed.remaining_size.as_deref(), Some("0"));
    }

    #[test]
    fn agg_trade_round_trips_wire_payload() {
        // Captured live from tradingapi.testnet.bullet.xyz @aggTrade.
        // Per the 2026-05-23 changelog the DEX-specific fields are emitted as
        // empty/zero placeholders — present on the wire, not omitted. Our
        // `Option<T>` wrap with `default` correctly deserializes these as
        // `Some(emptyvalue)`, confirming the placeholders parse without error.
        let wire = r#"{"e":"aggTrade","E":1779600413218736,"s":"SOL-USD","a":13205413,"p":"85.65","q":"0.01","f":13205413,"l":13205413,"T":1779600413211110,"m":true,"th":"0x94764a","ua":"","oi":0,"mk":false,"ff":false,"lq":false,"fe":"0","nf":"0","fa":"","co":0,"sd":"","ft":"","z":"0","Z":"0","rs":"0"}"#;
        let parsed: AggTradeMessage =
            serde_json::from_str(wire).expect("AggTradeMessage deserializes");
        // Non-deprecated fields populated:
        assert_eq!(parsed.symbol, "SOL-USD");
        assert_eq!(parsed.price, "85.65");
        assert_eq!(parsed.quantity, "0.01");
        assert!(parsed.is_buyer_maker);
        assert_eq!(parsed.tx_hash, "0x94764a");
        // Deprecated placeholders: present-but-meaningless. The point of these
        // assertions is to verify the empty/zero placeholders deserialize
        // without error, not that the values are useful.
        assert_eq!(parsed.user_address.as_deref(), Some(""));
        assert_eq!(parsed.order_id, Some(0));
        assert_eq!(parsed.is_maker, Some(false));
        assert_eq!(parsed.is_full_fill, Some(false));
        assert_eq!(parsed.fill_type.as_deref(), Some(""));
        assert_eq!(parsed.cumulative_filled_size.as_deref(), Some("0"));
        assert_eq!(parsed.remaining_size.as_deref(), Some("0"));
    }

    #[test]
    fn trade_fill_in_order_update_data() {
        let wire = r#"{"s":"SOL-USD","i":1,"X":"FILLED","x":"TRADE","T":1,"th":"0x","S":"BUY","l":"0.1","L":"85","n":"0","N":"USDC","m":false,"t":1,"rp":"0"}"#;
        let parsed: OrderUpdateData =
            serde_json::from_str(wire).expect("untagged variant resolves to TradeFill");
        match parsed {
            OrderUpdateData::TradeFill(_) => {}
            other => panic!("expected TradeFill, got {other:?}"),
        }
    }
}
