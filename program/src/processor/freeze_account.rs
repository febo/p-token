use pinocchio::{account_info::AccountInfo, ProgramResult};

use super::shared::toggle_account_state::process_toggle_account_state;

#[inline(never)]
pub fn process_freeze_account(accounts: &[AccountInfo]) -> ProgramResult {
    process_toggle_account_state(accounts, true)
}
