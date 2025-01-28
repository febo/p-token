use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, ProgramResult
};

use crate::entrypoint::process_instruction;

#[inline(always)]
pub fn process_batch(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Validates the instruction data.
    let (counter, mut instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let mut lengths: &[u8];
    let mut accounts = accounts;
    let mut current_accounts: &[AccountInfo];
    let mut current_instruction_data: &[u8];

    for _ in 0..*counter {
        (lengths, instruction_data) = instruction_data
            .split_at_checked(2)
            .ok_or(ProgramError::InvalidInstructionData)?;
        (current_accounts, accounts) = accounts.split_at_checked(lengths[0].into()).ok_or(ProgramError::InvalidInstructionData)?;
        (current_instruction_data, instruction_data) = instruction_data.split_at_checked(lengths[1].into()).ok_or(ProgramError::InvalidInstructionData)?;
        process_instruction(&token_interface::program::ID, current_accounts, current_instruction_data)?;
    }
    Ok(())
}