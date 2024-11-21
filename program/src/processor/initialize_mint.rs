use pinocchio::{account_info::AccountInfo, pubkey::Pubkey, ProgramResult};

use super::shared;

#[inline(never)]
pub fn process_initialize_mint(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    shared::initialize_mint::process_initialize_mint(accounts, instruction_data, true)
}
