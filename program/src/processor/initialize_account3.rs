use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult};

use super::shared;

#[inline(never)]
pub fn process_initialize_account3(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let owner = unsafe { &*(instruction_data.as_ptr() as *const Pubkey) };
    shared::initialize_account::process_initialize_account(program_id, accounts, Some(owner), false)
}
