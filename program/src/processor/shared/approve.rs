use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use token_interface::{
    error::TokenError,
    state::{account::Account, load, load_mut_unchecked, mint::Mint},
};

use crate::processor::validate_owner;

#[inline(always)]
pub fn process_approve(
    accounts: &[AccountInfo],
    amount: u64,
    expected_decimals: Option<u8>,
) -> ProgramResult {
    // Accounts expected depend on whether we have the mint `decimals` or not; when we have the
    // mint `decimals`, we expect the mint account to be present.

    let (source_account_info, expected_mint_info, delegate_info, owner_info, remaining) =
        if let Some(expected_decimals) = expected_decimals {
            let [source_account_info, expected_mint_info, delegate_info, owner_info, remaning @ ..] =
                accounts
            else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };

            (
                source_account_info,
                Some((expected_mint_info, expected_decimals)),
                delegate_info,
                owner_info,
                remaning,
            )
        } else {
            let [source_account_info, delegate_info, owner_info, remaning @ ..] = accounts else {
                return Err(ProgramError::NotEnoughAccountKeys);
            };
            (
                source_account_info,
                None,
                delegate_info,
                owner_info,
                remaning,
            )
        };

    // Validates source account.
    {
        // SAFETY: scoped immutable borrow of `source_account_info` account data. The `load`
        // validates that the account is initialized.
        let source_account =
            unsafe { load::<Account>(source_account_info.borrow_data_unchecked())? };

        if source_account.is_frozen() {
            return Err(TokenError::AccountFrozen.into());
        }

        if let Some((mint_info, expected_decimals)) = expected_mint_info {
            if mint_info.key() != &source_account.mint {
                return Err(TokenError::MintMismatch.into());
            }

            // SAFETY: scoped immutable borrow of `mint_info` account data. The `load`
            // validates that the account is initialized.
            let mint = unsafe { load::<Mint>(mint_info.borrow_data_unchecked())? };

            if expected_decimals != mint.decimals {
                return Err(TokenError::MintDecimalsMismatch.into());
            }
        }

        validate_owner(&source_account.owner, owner_info, remaining)?;
    }

    // Sets the delegate and delegated amount.

    // SAFETY: there is a single mutable borrow to `source_account_info` account data.
    // The account is also guaranteed to be initialized.
    let source_account =
        unsafe { load_mut_unchecked::<Account>(source_account_info.borrow_mut_data_unchecked())? };
    source_account.set_delegate(delegate_info.key());
    source_account.set_delegated_amount(amount);

    Ok(())
}
