use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DonationError {
    #[error("Invalid PDA for this user")]
    InvalidPDAUser,

    #[error("Invalid PDA for this program")]
    InvalidPDAProgram,

    #[error("Invalid program account")]
    InvalidProgramAccount,

    #[error("Admin required")]
    AdminRequired
}

impl From<DonationError> for ProgramError {
    fn from(e: DonationError) -> Self {
        ProgramError::Custom(e as u32)
    }
}