use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::error::DonationError;
use crate::selector::ProgramSelector;
use crate::storage::{DonationPDA, Bank};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8]
    ) -> ProgramResult {
        let selector = ProgramSelector::try_from_slice(data)?;
        match selector {
            ProgramSelector::Donate { amount } => Self::process_donate(accounts, amount),
            ProgramSelector::Withdraw => Self::process_withdraw(accounts)
        }
    }

    fn process_donate(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user = next_account_info(accounts_iter)?;
        let donation_pda = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;
        let program_account = next_account_info(accounts_iter)?;

        if !user.is_signer  {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !DonationPDA::is_ok_donation_pda_pubkey(user.key, donation_pda.key) {
            return Err(DonationError::InvalidPDA.into());
        }
        if !Bank::is_ok_bank_pubkey(bank.key) {
            return Err(DonationError::InvalidPDA.into());
        }

        let mut donation_pda = DonationPDA::try_from_slice(&donation_pda.data.borrow())?;
        let mut bank = Bank::try_from_slice(&bank.data.borrow())?;
        
        invoke(
            &system_instruction::transfer(user.key, program_account.key, amount),
            &[user.clone(), program_account.clone()],
        )?;

        donation_pda.total_donated += amount;
        bank.funds += amount;

        Ok(())
    }

    fn process_withdraw(accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let admin = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;
        let program_account = next_account_info(accounts_iter)?;

        if !admin.is_signer  {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        let mut bank = Bank::try_from_slice(&bank.data.borrow())?;

        if bank.admin != admin.key.to_bytes() && bank.admin != [0; 32] {
            return Err(DonationError::AdminRequired.into());
        } 
        
        invoke(
            &system_instruction::transfer(program_account.key, admin.key, bank.funds),
            &[program_account.clone(), admin.clone()],
        )?;

        bank.funds = 0;

        Ok(())
    }
}