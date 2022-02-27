use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, DONATION_PDA_SEED, BANK_PDA_SEED};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DonationPDA {
    pub total_donated: u64
}

impl DonationPDA {
    pub fn get_donation_pda_pubkey(user: &Pubkey) -> Pubkey {
        Pubkey::create_with_seed(user, DONATION_PDA_SEED, &id()).unwrap()
    }

    pub fn is_ok_donation_pda_pubkey(user: &Pubkey, donation_pda: &Pubkey) -> bool {
        donation_pda.to_bytes() == Self::get_donation_pda_pubkey(user).to_bytes()
    }
} 

/// There is only one bank account. All donation accounts use it.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Bank {
    /// Only admin can change this account
    pub admin: [u8; 32],

    /// Available to withdraw amount
    pub funds: u64,

    // List of all users
    // ???
}

impl Bank {
    pub fn get_bank_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[BANK_PDA_SEED.as_bytes()], &id())
    }

    pub fn get_bank_pubkey() -> Pubkey {
        let (pubkey, _) = Self::get_bank_pubkey_with_bump();
        pubkey
    }

    pub fn is_ok_bank_pubkey(bank_pubkey: &Pubkey) -> bool {
        let (pubkey, _) = Self::get_bank_pubkey_with_bump();
        pubkey.to_bytes() == bank_pubkey.to_bytes()
    }
}