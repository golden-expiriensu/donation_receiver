pub mod processor;
pub mod selector;
pub mod storage;
pub mod error;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const DONATION_PDA_SEED: &str = "donation";
solana_program::declare_id!("9onZvMzqAFzSHJrLNVWfqLRFFQ5ZCGzNXB4PBxmp6z5Y");
