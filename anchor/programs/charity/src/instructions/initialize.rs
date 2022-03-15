use anchor_lang::prelude::*;
use crate::state::bank::Bank;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=admin)]
    pub bank: Account<'info, Bank>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}