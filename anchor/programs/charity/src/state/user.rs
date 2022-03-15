use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub total_donated: u64
}