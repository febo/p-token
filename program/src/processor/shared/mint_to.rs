use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use token_interface::{
    error::TokenError,
    state::{account::Account, load, load_mut_unchecked, mint::Mint},
};

use crate::processor::{check_account_owner, validate_owner};

#[inline(always)]
pub fn process_mint_to(
    accounts: &[AccountInfo],
    amount: u64,
    expected_decimals: Option<u8>,
) -> ProgramResult {
    let [mint_info, destination_account_info, owner_info, remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validates the destination account.
    {
        // SAFETY: scoped immutable borrow of `destination_account_info` account data. When
        // `owner_info` is the same as `destination_account_info`, there will be another immutable
        // borrow in `validate_owner` – this is safe because both borrows are immutable.
        let destination_account =
            unsafe { load::<Account>(destination_account_info.borrow_data_unchecked())? };

        if destination_account.is_frozen() {
            return Err(TokenError::AccountFrozen.into());
        }

        if destination_account.is_native() {
            return Err(TokenError::NativeNotSupported.into());
        }

        if mint_info.key() != &destination_account.mint {
            return Err(TokenError::MintMismatch.into());
        }

        // SAFETY: scoped immutable borrow of `mint_info` account data. When `owner_info` is the
        // same as `mint_info`, there will be another immutable borrow in `validate_owner` – this
        // is safe because both borrows are immutable.
        let mint = unsafe { load::<Mint>(mint_info.borrow_data_unchecked())? };

        if let Some(expected_decimals) = expected_decimals {
            if expected_decimals != mint.decimals {
                return Err(TokenError::MintDecimalsMismatch.into());
            }
        }

        match mint.mint_authority() {
            Some(mint_authority) => validate_owner(mint_authority, owner_info, remaining)?,
            None => return Err(TokenError::FixedSupply.into()),
        }
    }

    if amount == 0 {
        check_account_owner(mint_info)?;
        check_account_owner(destination_account_info)?;
    } else {
        // SAFETY: there is a single mutable borrow to `destination_account_info`. The 'mint_info'
        // is guaranteed to be different than 'destination_account_info' since there is a length check
        // on `load`.
        let destination_account = unsafe {
            load_mut_unchecked::<Account>(destination_account_info.borrow_mut_data_unchecked())?
        };
        let destination_amount = destination_account
            .amount()
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;
        destination_account.set_amount(destination_amount);

        // SAFETY: there is a single mutable borrow to `mint_info`.
        let mint = unsafe { load_mut_unchecked::<Mint>(mint_info.borrow_mut_data_unchecked())? };
        let mint_supply = mint
            .supply()
            .checked_add(amount)
            .ok_or(TokenError::Overflow)?;
        mint.set_supply(mint_supply);
    }

    Ok(())
}
