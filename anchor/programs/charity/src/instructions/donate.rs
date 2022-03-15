use anchor_lang::prelude::*;
use crate::state::{
    user::User,
    bank::Bank
};

#[derive(Accounts)]
pub struct Donate<'info> {
    pub user_pda: Account<'info, User>,
    pub bank: Account<'info, Bank>,
    pub user: Signer<'info>
}