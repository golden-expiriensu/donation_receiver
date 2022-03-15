use anchor_lang::prelude::*;

#[account]
pub struct Bank {
    pub admin: [u8; 32],
    pub funds: u64,
    pub donaters: Vec<[u8; 32]>
}

impl Default for Bank {
    fn default() -> Self {
        Bank {
            admin: [0; 32],
            funds: 0,
            donaters: Vec::new()
        }
    }
}