use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::Display;

/// Error codes aligned with Binance API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[repr(i32)]
pub enum ErrorCode {
    // general errors
    #[strum(serialize = "Unknown error")]
    Unknown = -1000,
    #[strum(serialize = "Disconnected")]
    Disconnected = -1001,
    #[strum(serialize = "Unauthorized")]
    Unauthorized = -1002,
    #[strum(serialize = "Too many requests")]
    TooManyRequests = -1003,
    #[strum(serialize = "Unexpected response")]
    UnexpectedResponse = -1006,
    #[strum(serialize = "Timeout")]
    Timeout = -1007,
    #[strum(serialize = "Unknown order")]
    UnknownOrder = -1014,
    #[strum(serialize = "Too many orders")]
    TooManyOrders = -1015,
    #[strum(serialize = "Service unavailable")]
    ServiceUnavailable = -1016,
    #[strum(serialize = "Unsupported operation")]
    UnsupportedOperation = -1020,
    #[strum(serialize = "Invalid timestamp")]
    InvalidTimestamp = -1021,
    #[strum(serialize = "Invalid signature")]
    InvalidSignature = -1022,
    #[strum(serialize = "Mandatory parameter missing")]
    MandatoryParamMissing = -1102,
    #[strum(serialize = "Bad precision")]
    BadPrecision = -1111,
    #[strum(serialize = "Invalid order type")]
    InvalidOrderType = -1116,
    #[strum(serialize = "Invalid side")]
    InvalidSide = -1117,
    #[strum(serialize = "Invalid symbol")]
    InvalidSymbol = -1122,

    // custom error for when address is not passed or invalid
    #[strum(serialize = "Invalid user address")]
    InvalidUserAddress = -1123,

    // order errors
    #[strum(serialize = "New order rejected")]
    NewOrderRejected = -2010,
    #[strum(serialize = "Cancel rejected")]
    CancelRejected = -2011,
    #[strum(serialize = "No such order")]
    NoSuchOrder = -2013,
    #[strum(serialize = "API key format invalid")]
    ApiKeyFormatInvalid = -2014,
    #[strum(serialize = "Invalid API key/IP/permissions")]
    InvalidApiKeyIpPermissions = -2015,
    #[strum(serialize = "Order would immediately trigger")]
    OrderWouldTrigger = -2021,

    // subscription errors (custom codes)
    #[strum(serialize = "Invalid subscription format")]
    InvalidSubscriptionFormat = -1004,
    #[strum(serialize = "Symbol not found")]
    SymbolNotFound = -1005,
    #[strum(serialize = "Validation error")]
    ValidationError = -1008,
    #[strum(serialize = "Subscription already exists")]
    SubscriptionExists = -1010,

    // server errors (internal)
    #[strum(serialize = "Client not found")]
    ClientNotFound = -4001,
    #[strum(serialize = "Could not send message")]
    CouldNotSendMessage = -4002,
}

impl Serialize for ErrorCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = i32::deserialize(deserializer)?;
        Ok(match code {
            -1000 => ErrorCode::Unknown,
            -1001 => ErrorCode::Disconnected,
            -1002 => ErrorCode::Unauthorized,
            -1003 => ErrorCode::TooManyRequests,
            -1004 => ErrorCode::InvalidSubscriptionFormat,
            -1005 => ErrorCode::SymbolNotFound,
            -1006 => ErrorCode::UnexpectedResponse,
            -1007 => ErrorCode::Timeout,
            -1008 => ErrorCode::ValidationError,
            -1010 => ErrorCode::SubscriptionExists,
            -1014 => ErrorCode::UnknownOrder,
            -1015 => ErrorCode::TooManyOrders,
            -1016 => ErrorCode::ServiceUnavailable,
            -1020 => ErrorCode::UnsupportedOperation,
            -1021 => ErrorCode::InvalidTimestamp,
            -1022 => ErrorCode::InvalidSignature,
            -1102 => ErrorCode::MandatoryParamMissing,
            -1111 => ErrorCode::BadPrecision,
            -1116 => ErrorCode::InvalidOrderType,
            -1117 => ErrorCode::InvalidSide,
            -1122 => ErrorCode::InvalidSymbol,
            -1123 => ErrorCode::InvalidUserAddress,
            -2010 => ErrorCode::NewOrderRejected,
            -2011 => ErrorCode::CancelRejected,
            -2013 => ErrorCode::NoSuchOrder,
            -2014 => ErrorCode::ApiKeyFormatInvalid,
            -2015 => ErrorCode::InvalidApiKeyIpPermissions,
            -2021 => ErrorCode::OrderWouldTrigger,
            -4001 => ErrorCode::ClientNotFound,
            -4002 => ErrorCode::CouldNotSendMessage,
            _ => ErrorCode::Unknown,
        })
    }
}

/// WebSocket error for both internal handling and client responses.
/// Serializes as `{"param": "...", "code": N, "msg": "..."}` for JSON responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSError {
    #[serde(skip_serializing_if = "Option::is_none")]
    param: Option<String>,
    code: ErrorCode,
    #[serde(rename = "msg")]
    message: String,
}

impl std::error::Error for WSError {}

impl std::fmt::Display for WSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl WSError {
    /// Create error with default message from code
    pub fn new(code: ErrorCode) -> Self {
        Self { code, message: code.to_string(), param: None }
    }

    /// Create error with custom message
    pub fn with_message(code: ErrorCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into(), param: None }
    }

    /// Create error with param context (for atomic batch failures)
    pub fn with_param(
        code: ErrorCode,
        param: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self { code, message: message.into(), param: Some(param.into()) }
    }

    /// Get the error code
    pub fn error_code(&self) -> ErrorCode {
        self.code
    }

    /// Get the numeric error code
    pub fn code(&self) -> i32 {
        self.code as i32
    }

    /// Get the param that caused the error
    pub fn param(&self) -> Option<&str> {
        self.param.as_deref()
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    // Convenience constructors
    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self::with_message(ErrorCode::ValidationError, msg)
    }

    pub fn invalid_subscription(msg: impl Into<String>) -> Self {
        Self::with_message(ErrorCode::InvalidSubscriptionFormat, msg)
    }

    pub fn invalid_subscription_with_param(
        param: impl Into<String>,
        msg: impl Into<String>,
    ) -> Self {
        Self::with_param(ErrorCode::InvalidSubscriptionFormat, param, msg)
    }

    pub fn symbol_not_found(symbol: &str) -> Self {
        Self::with_param(ErrorCode::InvalidSymbol, symbol, format!("symbol not found: {symbol}"))
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::with_message(ErrorCode::Unauthorized, msg)
    }

    pub fn unauthorized_with_param(param: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::with_param(ErrorCode::Unauthorized, param, msg)
    }

    pub fn subscription_exists(param: impl Into<String>) -> Self {
        Self::with_param(ErrorCode::SubscriptionExists, param, "subscription already exists")
    }

    pub fn mandatory_param_missing(msg: impl Into<String>) -> Self {
        Self::with_message(ErrorCode::MandatoryParamMissing, msg)
    }

    pub fn server_busy(msg: impl Into<String>) -> Self {
        Self::with_message(ErrorCode::Disconnected, msg)
    }
}
