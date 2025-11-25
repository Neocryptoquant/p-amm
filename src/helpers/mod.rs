pub mod deposit;
pub mod withdraw;
pub mod swap;

pub use deposit::*;
use pinocchio::program_error::ProgramError;
pub use withdraw::*;
pub use swap::*;

pub enum AmmHelpers {
    Deposit = 0,
    Withdraw = 1,
    Swap = 2,
}


impl TryFrom<u8> for AmmHelpers {
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AmmHelpers::Deposit),
            1 => Ok(AmmHelpers::Withdraw),
            2 => Ok(AmmHelpers::Swap),
            _ => Err(ProgramError::InvalidInstructionData),
        }   
    }
}