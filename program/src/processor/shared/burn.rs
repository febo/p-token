use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use token_interface::{
    error::TokenError,
    state::{account::Account, load, load_mut_unchecked, mint::Mint},
};

use crate::processor::{check_account_owner, validate_owner};

#[inline(always)]
pub fn process_burn(
    accounts: &[AccountInfo],
    amount: u64,
    expected_decimals: Option<u8>,
) -> ProgramResult {
    let [source_account_info, mint_info, authority_info, remaining @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (updated_source_amount, delegated_amount) = {
        // SAFETY: scoped immutable borrow of `source_account_info` account data. The `load`
        // validates that the account is initialized.
        let source_account =
            unsafe { load::<Account>(source_account_info.borrow_data_unchecked())? };

        if source_account.is_frozen() {
            return Err(TokenError::AccountFrozen.into());
        }
        if source_account.is_native() {
            return Err(TokenError::NativeNotSupported.into());
        }

        // Ensure the source account has the sufficient amount. This is done before
        // the value is updated on the account.
        let updated_source_amount = source_account
            .amount()
            .checked_sub(amount)
            .ok_or(TokenError::InsufficientFunds)?;

        // SAFETY: scoped immutable borrow of `mint_info` account data. The `load`
        // validates that the account is initialized.
        let mint = unsafe { load::<Mint>(mint_info.borrow_data_unchecked())? };

        if mint_info.key() != &source_account.mint {
            return Err(TokenError::MintMismatch.into());
        }

        if let Some(expected_decimals) = expected_decimals {
            if expected_decimals != mint.decimals {
                return Err(TokenError::MintDecimalsMismatch.into());
            }
        }

        (
            updated_source_amount,
            if !source_account.is_owned_by_system_program_or_incinerator() {
                match source_account.delegate() {
                    Some(delegate) if authority_info.key() == delegate => {
                        validate_owner(delegate, authority_info, remaining)?;

                        Some(
                            source_account
                                .delegated_amount()
                                .checked_sub(amount)
                                .ok_or(TokenError::InsufficientFunds)?,
                        )
                    }
                    _ => {
                        validate_owner(&source_account.owner, authority_info, remaining)?;

                        None
                    }
                }
            } else {
                None
            },
        )
    };

    // Updates the source account and mint supply.

    // SAFETY: there are no active borrows of `source_account_info` and `mint_info` account data
    // at this point. They are also guaranteed to be initialized and different accounts.
    let (source_account, mint) = unsafe {
        (
            load_mut_unchecked::<Account>(source_account_info.borrow_mut_data_unchecked())?,
            load_mut_unchecked::<Mint>(mint_info.borrow_mut_data_unchecked())?,
        )
    };

    // Updated the delegated amount if necessary.
    if let Some(delegated_amount) = delegated_amount {
        source_account.set_delegated_amount(delegated_amount);

        if delegated_amount == 0 {
            source_account.clear_delegate();
        }
    }

    if amount == 0 {
        check_account_owner(source_account_info)?;
        check_account_owner(mint_info)?;
    } else {
        source_account.set_amount(updated_source_amount);

        let mint_supply = mint
            .supply()
            .checked_sub(amount)
            .ok_or(TokenError::Overflow)?;
        mint.set_supply(mint_supply);
    }

    Ok(())
}
