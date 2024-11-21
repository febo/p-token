use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use super::shared;

#[inline(never)]
pub fn process_initialize_multisig2(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let m = instruction_data
        .first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    shared::initialize_multisig::process_initialize_multisig(accounts, *m, false)
}
