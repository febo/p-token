use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use token_interface::{
    error::TokenError,
    state::{account::Account, load, load_mut_unchecked},
};

use super::validate_owner;

#[inline(always)]
pub fn process_revoke(accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
    let [source_account_info, owner_info, remaning @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    {
        // SAFETY: scoped immutable borrow of the `source_account` account data. The `load`
        // validates that the source account is initialized.
        let source_account =
            unsafe { load::<Account>(source_account_info.borrow_data_unchecked())? };

        if source_account.is_frozen() {
            return Err(TokenError::AccountFrozen.into());
        }

        validate_owner(&source_account.owner, owner_info, remaning)?;
    }

    // SAFETY: single mutable borrow of the `source_account_info` account data. The
    // `source_account_info` is guaranteed to be initialized.
    let source_account =
        unsafe { load_mut_unchecked::<Account>(source_account_info.borrow_mut_data_unchecked())? };
    source_account.clear_delegate();
    source_account.set_delegated_amount(0);

    Ok(())
}
