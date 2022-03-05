use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::{id, storage::{ Bank, DonationPDA }};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramSelector {
    /// Initialization method
    /// Accounts:
    /// 0. `[signer, writable]` admin
    /// 1. `[writable]` Bank, program pda
    /// 2. `[]` Rent sysvar
    /// 3. `[]` System program
    TransferOwnership { new_admin: [u8; 32] },
    
    /// Make a donation to the platform
    /// Accounts:
    /// 0. `[signer, writable]` Debit lamports from this account
    /// 1. `[writable]` User donation pda
    /// 2. `[writable]` Bank, program pda
    /// 3. `[writable]` System program, credit lamports to this account
    Donate { amount: u64 },

    /// Withdraw all money from the platform
    /// Accounts:
    /// 0. `[signer, writable]` An admin account, credit lamports to this account
    /// 1. `[writable]` Bank, program pda
    /// 2. `[writable]` System program, debit lamports from this account
    Withdraw
}


impl ProgramSelector {
    pub fn transfer_ownership(
        admin: &Pubkey,
        new_admin: [u8; 32]
    ) -> Instruction {
        Instruction::new_with_borsh(
            id(),
            &ProgramSelector::TransferOwnership { new_admin },
            vec![
                AccountMeta::new(*admin, true),
                AccountMeta::new(Bank::get_bank_pubkey(), false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(id(), false)
            ]
        )
    }

    pub fn donate(
        user: &Pubkey,
        amount: u64
    ) -> Instruction {
        Instruction::new_with_borsh(
            id(),
            &ProgramSelector::Donate { amount },
            vec![
                AccountMeta::new(*user, true),
                AccountMeta::new(DonationPDA::get_donation_pda_pubkey(&user), false),
                AccountMeta::new(Bank::get_bank_pubkey(), false),
                AccountMeta::new(id(), false)
            ]
        )
    }

    pub fn withdraw(admin: &Pubkey) -> Instruction {
        Instruction::new_with_borsh(
            id(),
            &ProgramSelector::Withdraw,
            vec![
                AccountMeta::new(*admin, true),
                AccountMeta::new(Bank::get_bank_pubkey(), false),
                AccountMeta::new(id(), false)
            ]
        )
    }
}
