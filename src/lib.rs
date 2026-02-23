//! Shared types for the Trading API WebSocket protocol.
//!
//! This crate contains the message types used for WebSocket communication
//! between clients and the trading API server.

mod client;
mod error;
mod request;
mod server;

pub use client::*;
pub use error::*;
pub use request::*;
pub use server::*;
