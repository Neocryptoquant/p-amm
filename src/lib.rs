use pinocchio::{ProgramResult, account_info::AccountInfo, pubkey::Pubkey, entrypoint};

mod state;
mod helpers;
mod tests;

entrypoint!(process_instruction);
pinocchio_pubkey::declare_id!("PAMMGL6h3EeQmwpNMAzjJtjXsaryYrY6JJCUB4QxFqH");
