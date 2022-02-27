use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramSelector {
    /// Make a donation to the platform
    /// Accounts:
    /// 0. `[signer, writable]` Debit lamports from this account
    /// 1. `[writable]` User donation pda
    /// 2. `[writable]` Bank, program pda
    /// 3. `[writable]` System program, credit lamports to this account
    Donate { amount: u64 },

    /// Withdraw all money from the platform
    /// Accounts:
    /// 0. `[signer, writable]` An owner account, credit lamports to this account
    /// 1. `[writable]` Bank, program pda
    /// 2. `[writable]` System program, debit lamports from this account
    Withdraw
}