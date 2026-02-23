use std::fmt;

use serde::{Deserialize, Serialize};

/// Request ID for matching responses to requests (Binance uses integers)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestId(u64);

impl RequestId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for RequestId {
    fn from(n: u64) -> Self {
        RequestId(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_id_serde() {
        let id: RequestId = serde_json::from_str("123").unwrap();
        assert_eq!(id, RequestId(123));

        let id = RequestId(456);
        assert_eq!(serde_json::to_string(&id).unwrap(), "456");

        assert_eq!(RequestId(42).to_string(), "42");
    }
}
