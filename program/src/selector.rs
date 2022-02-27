use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramSelector {
    /// Make a donation to the platform
    /// Accounts:
    /// 0. `[signer, writable]` Debit lamports from this account
    /// 1. `[writable]` Credit lamports to this account
    /// 2. `[]` System program
    Donate,

    /// Withdraw all money from the platform
    /// Accounts:
    /// 0. `[signer, writable]`
    /// 1. `[]` System program
    Withdraw
}