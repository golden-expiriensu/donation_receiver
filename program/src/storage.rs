use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, DONATION_PDA_SEED};

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