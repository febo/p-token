use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::entrypoint::process_instruction;

/// The size of the batch instruction header.
///
/// The header of each instruction consists of two `u8` values:
///  * number of the accounts
///  * length of the instruction data
const IX_HEADER_SIZE: usize = 2;

pub fn process_batch(mut accounts: &[AccountInfo], mut instruction_data: &[u8]) -> ProgramResult {
    loop {
        // Validates the instruction data and accounts offset.

        match instruction_data.len() {
            0 => break,
            n if n < IX_HEADER_SIZE => {
                // The instruction data must have at least two bytes.
                return Err(ProgramError::InvalidInstructionData);
            }
            _ => (),
        }
        // SAFETY: The instruction data is guaranteed to have at least two bytes.
        let expected_accounts = unsafe { *instruction_data.get_unchecked(0) as usize };
        let data_offset = IX_HEADER_SIZE + unsafe { *instruction_data.get_unchecked(1) as usize };
        if instruction_data.len() < data_offset {
            return Err(ProgramError::InvalidInstructionData);
        }

        if accounts.len() < expected_accounts {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        // Process the instruction.

        process_instruction(
            &token_interface::program::ID,
            &accounts[..expected_accounts],
            &instruction_data[IX_HEADER_SIZE..data_offset],
        )?;

        accounts = &accounts[expected_accounts..];
        instruction_data = &instruction_data[data_offset..];
    }

    Ok(())
}
