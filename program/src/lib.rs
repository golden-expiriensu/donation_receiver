pub mod processor;
pub mod instruction;
pub mod state;
pub mod error;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const DONATION_PDA_SEED: &str = "donation";
pub const BANK_PDA_SEED: &str = "bank";

solana_program::declare_id!("8qGenigMJogtv3yFcUSCK3dyQiWTYi6kuydumJTscDh9");
