use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    {system_instruction, msg},
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed
};


use crate::error::DonationError;
use crate::instruction::ProgramSelector;
use crate::state::{DonationPDA, Bank};
use crate::{id, BANK_PDA_SEED};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8]
    ) -> ProgramResult {
        let selector = ProgramSelector::try_from_slice(data)?;
        match selector {
            ProgramSelector::TransferOwnership { new_admin } => Self::transfer_ownership(accounts, new_admin),
            ProgramSelector::Donate { amount } => Self::process_donate(accounts, amount),
            ProgramSelector::Withdraw => Self::process_withdraw(accounts)
        }
    }

    fn transfer_ownership(accounts: &[AccountInfo], new_admin: [u8; 32]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let admin = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;
        let rent = next_account_info(accounts_iter)?;
        let system = next_account_info(accounts_iter)?;

        if !Bank::is_ok_bank_pubkey(bank.key) {
            return Err(DonationError::InvalidProgramAccount.into());
        }
        if !admin.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if bank.data_is_empty() {
            let bank_instance = Bank { admin: admin.key.to_bytes(), funds: 0, donaters: Vec::new() };
            let allocated_space = bank_instance.try_to_vec()?.len();
            let rent = &Rent::from_account_info(rent)?;
            let lamports = rent.minimum_balance(allocated_space);
            let signer_seeds: &[&[_]] = &[BANK_PDA_SEED.as_bytes(), &id().to_bytes()];
            msg!("{:?}", bank.key.to_string());

            invoke_signed(
                &system_instruction::create_account(
                    admin.key,
                    bank.key,
                    lamports,
                    allocated_space as u64,
                    &id(),
                ),
                &[admin.clone(), bank.clone(), system.clone()],
                &[&signer_seeds],
            )?;
        }

        let mut bank = Bank::try_from_slice(&bank.data.borrow())?;

        if bank.admin != admin.key.to_bytes() && bank.admin != [0; 32] {
            return Err(DonationError::AdminRequired.into());
        }

        bank.admin = new_admin;            
        
        Ok(())
    }

    fn process_donate(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user = next_account_info(accounts_iter)?;
        let donation_pda = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;
        let program = next_account_info(accounts_iter)?;

        if !user.is_signer  {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !DonationPDA::is_ok_donation_pda_pubkey(user.key, donation_pda.key) {
            return Err(DonationError::InvalidPDAUser.into());
        }
        if !Bank::is_ok_bank_pubkey(bank.key) {
            return Err(DonationError::InvalidPDAProgram.into());
        }
        if program.key.to_bytes() != id().to_bytes() {
            return Err(DonationError::InvalidProgramAccount.into());
        }

        let mut donation_pda = DonationPDA::try_from_slice(&donation_pda.data.borrow())?;
        let mut bank = Bank::try_from_slice(&bank.data.borrow())?;
        
        invoke(
            &system_instruction::transfer(user.key, program.key, amount),
            &[user.clone(), program.clone()],
        )?;

        if donation_pda.total_donated == 0 && amount > 0 {
            bank.donaters.push(user.key.to_bytes())
        }

        donation_pda.total_donated += amount;
        bank.funds += amount;

        Ok(())
    }

    fn process_withdraw(accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let admin = next_account_info(accounts_iter)?;
        let bank = next_account_info(accounts_iter)?;
        let program = next_account_info(accounts_iter)?;

        if !admin.is_signer  {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !Bank::is_ok_bank_pubkey(bank.key) {
            return Err(DonationError::InvalidPDAProgram.into());
        }
        
        let mut bank = Bank::try_from_slice(&bank.data.borrow())?;

        if bank.admin != admin.key.to_bytes() && bank.admin != [0; 32] {
            return Err(DonationError::AdminRequired.into());
        } 
        if program.key.to_bytes() != id().to_bytes() {
            return Err(DonationError::InvalidProgramAccount.into());
        }
        
        invoke(
            &system_instruction::transfer(program.key, admin.key, bank.funds),
            &[program.clone(), admin.clone()],
        )?;

        bank.funds = 0;

        Ok(())
    }
}