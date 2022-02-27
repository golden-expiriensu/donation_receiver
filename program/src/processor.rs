use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::{rent::Rent, Sysvar};
use solana_program::{msg, system_instruction};

use crate::error::DonationError;
use crate::selector::ProgramSelector;
use crate::storage::{DonationPDA};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8]
    ) -> ProgramResult {
        let selector = ProgramSelector::try_from_slice(data)?;
        match selector {
            ProgramSelector::Donate => Self::process_donate(accounts),
            ProgramSelector::Withdraw => Self::process_withdraw(accounts)
        }
    }

    fn process_donate(accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user = next_account_info(accounts_iter)?;
        let donation_pda = next_account_info(accounts_iter)?;

        if !user.is_signer  {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !DonationPDA::is_ok_donation_pda_pubkey(user.key, donation_pda.key) {
            return Err(DonationError::InvalidPDA.into());
        }

        let mut donation_pda = DonationPDA::try_from_slice(&donation_pda.data.borrow())?;

        donation_pda.total_donated += 55;
        // TODO: implement receive

        Ok(())
    }

    fn process_withdraw(accounts: &[AccountInfo]) -> ProgramResult {
        Ok(())
    }
}