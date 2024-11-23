use core::str;

use pinocchio::{
    account_info::AccountInfo, program::set_return_data, program_error::ProgramError, ProgramResult,
};
use token_interface::{error::TokenError, state::mint::Mint};

use super::{check_account_owner, try_ui_amount_into_amount};

#[inline(always)]
pub fn process_ui_amount_to_amount(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ui_amount =
        str::from_utf8(instruction_data).map_err(|_error| ProgramError::InvalidInstructionData)?;

    let mint_info = accounts.first().ok_or(ProgramError::NotEnoughAccountKeys)?;
    check_account_owner(mint_info)?;

    let mint =
        bytemuck::try_from_bytes_mut::<Mint>(unsafe { mint_info.borrow_mut_data_unchecked() })
            .map_err(|_error| TokenError::InvalidMint)?;

    let amount = try_ui_amount_into_amount(ui_amount.to_string(), mint.decimals)?;
    set_return_data(&amount.to_le_bytes());

    Ok(())
}
