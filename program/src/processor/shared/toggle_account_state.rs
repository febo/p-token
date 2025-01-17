use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use token_interface::{
    error::TokenError,
    state::{account::Account, account_state::AccountState, load, load_mut_unchecked, mint::Mint},
};

use crate::processor::validate_owner;

#[inline(always)]
pub fn process_toggle_account_state(accounts: &[AccountInfo], freeze: bool) -> ProgramResult {
    let [source_account_info, mint_info, authority_info, remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    {
        // SAFETY: scoped immutable borrow of `source_account_info` account data. The `load`
        // validates that the account is initialized.
        let source_account =
            unsafe { load::<Account>(source_account_info.borrow_data_unchecked())? };

        if freeze && source_account.is_frozen() || !freeze && !source_account.is_frozen() {
            return Err(TokenError::InvalidState.into());
        }
        if source_account.is_native() {
            return Err(TokenError::NativeNotSupported.into());
        }
        if mint_info.key() != &source_account.mint {
            return Err(TokenError::MintMismatch.into());
        }

        // SAFETY: scoped immutable borrow of `mint_info` account data. The `load`
        // validates the data length and that the mint is initialized.
        let mint = unsafe { load::<Mint>(mint_info.borrow_data_unchecked())? };

        match mint.freeze_authority() {
            Some(authority) => validate_owner(authority, authority_info, remaining),
            None => Err(TokenError::MintCannotFreeze.into()),
        }?;
    }

    // SAFETY: there is a single mutable borrow to `source_account_info`.
    let source_account =
        unsafe { load_mut_unchecked::<Account>(source_account_info.borrow_mut_data_unchecked())? };

    source_account.state = if freeze {
        AccountState::Frozen
    } else {
        AccountState::Initialized
    };

    Ok(())
}
