use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    msg,
    entrypoint::ProgramResult, 
    pubkey::Pubkey,
};

use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("EntryPoint OK");
    Processor::process( accounts, instruction_data)
}