use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;

use instructions::{initialize::*, donate::*};
use error::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod charity {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bank = &mut ctx.accounts.bank;
        bank.admin = ctx.accounts.admin.key().to_bytes();
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let user = ctx.accounts.user.key.to_bytes();

        let bank = &mut ctx.accounts.bank;
        let user_pda = &mut ctx.accounts.user_pda;

        require!(amount > 0, DonationError::ZeroDonation);
        // ? TODO: как блеать ? require!(acc.user.key().to_bytes() == acc.user_pda.user)

        if (user_pda.total_donated == 0) {
            bank.donaters.push(user);
        }
        bank.funds += amount;
        user_pda.total_donated += amount;

        // TODO: transfer from user to bank

        Ok(())
    }
}
