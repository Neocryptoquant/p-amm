use pinocchio::{
    account_info::AccountInfo, instruction::{AccountMeta, Instruction, Seed}, msg, program_error::ProgramError, pubkey::Pubkey
};
use spl_token_interface::instruction::transfer;

use crate::state::Config;

pub fn process_deposit_helper(
    accounts: &[AccountInfo],
    data: &[u8]) -> ProgramResult {
        msg!("Processing deposit helper");

        /// 0xAbim: Account needed for TransferChecked:
        /// [0]: Maker (signer)
        /// [1]: Mint A
        /// [5]: Authority of Mint A
        /// [6]: Maker's Associated Token Account for Mint A
        /// [2]: Minter
        /// [7]: Destination Token Account (PAMM's Token Account for Mint A)
        /// [11]: Token Program
        let [
            maker,
            mint_a,
            mint_b,
            maker_ata_a,
            system_program,
            token_program,
            _associated_token_program,,
            _rent @..,
        ] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
    
    let maker = spl_token_interface::state::Account::from_account_info(&maker)?;
    if maker.owner != maker.key() {
        return Err(ProgramError::IllegalOwner);
    }



    Ok(())
    }