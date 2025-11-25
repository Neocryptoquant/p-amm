use pinocchio::{
    account_info::{self, AccountInfo},
    program_error::ProgramError, pubkey::Pubkey,
};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Config{
    seed: [u8; 8],
    authority: [u8; 32],
    mint_x: [u8; 32],
    mint_y: [u8; 32],
    fee: [u8; 8],
    locked: bool,
    config_bump: [u8; 1],
    lp_bump: [u8; 1],
}

impl Config {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 1 + 1 + 1;
    pub fn from_account_info(account_info: &AccountInfo) -> Result<&mut Self, ProgramError>{
        let mut data = account_info.try_borrow_mut_data()?;
        // 0xAbim: Used Self instead of Config to avoid hardcoding the struct name
        if data.len() != Config::LEN {
        return Err(ProgramError::InvalidAccountData);
        }

        if (data.as_ptr() as usize) % core::mem::align_of::<Self>() != 0 {
        return Err(ProgramError::InvalidAccountData);
        }
    Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) } )
     }

     pub fn seed(&self) -> u64 {
        u64::from_le_bytes(self.seed)
     }
    
    pub fn authority(&self) -> Pubkey {
        Pubkey::from(self.authority)
    }

    pub fn mint_x(&self) -> Pubkey {
        Pubkey::from(self.mint_x)
    }

    pub fn mint_y(&self) -> Pubkey {
        Pubkey::from(self.mint_y)
    }

    pub fn fee(&self) -> u64 {
        u64::from_le_bytes(self.fee)
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }
    // 0xAbim: mimicked the existing fundraiser code style
    // the fundraiser is an in earlier repo of mine.: q3-amm
    
}
