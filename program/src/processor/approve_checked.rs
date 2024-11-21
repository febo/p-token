use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use super::shared;

#[inline(never)]
pub fn process_approve_checked(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() != 9 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = unsafe { (instruction_data.as_ptr() as *const u64).read_unaligned() };
    let decimals = unsafe { *instruction_data.as_ptr().add(8) };

    shared::approve::process_approve(program_id, accounts, amount, Some(decimals))
}
