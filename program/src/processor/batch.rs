use core::mem::MaybeUninit;

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use crate::entrypoint::inner_process_instruction;

macro_rules! write_account {
    ( $index_source:expr, $source:ident, $index_target:literal, $target:ident ) => {
        // TODO: need to validate that the indices are within bounds.
        unsafe {
            $target
                .get_unchecked_mut($index_target)
                .write($source.get_unchecked($index_source).clone())
        }
    };
}

macro_rules! fill_accounts {
    ( $indices:ident, $accounts:ident, $instruction_accounts:ident ) => {
        match $indices.len() {
            1 => {
                write_account!($indices[0] as usize, $accounts, 0, $instruction_accounts);
            }
            2 => {
                write_account!($indices[0] as usize, $accounts, 0, $instruction_accounts);
                write_account!($indices[1] as usize, $accounts, 1, $instruction_accounts);
            }
            3 => {
                write_account!($indices[0] as usize, $accounts, 0, $instruction_accounts);
                write_account!($indices[1] as usize, $accounts, 1, $instruction_accounts);
                write_account!($indices[2] as usize, $accounts, 2, $instruction_accounts);
            }
            4 => {
                write_account!($indices[0] as usize, $accounts, 0, $instruction_accounts);
                write_account!($indices[1] as usize, $accounts, 1, $instruction_accounts);
                write_account!($indices[2] as usize, $accounts, 2, $instruction_accounts);
                write_account!($indices[3] as usize, $accounts, 3, $instruction_accounts);
            }
            // TODO: Add more cases up to 15.
            _ => return Err(ProgramError::NotEnoughAccountKeys),
        }
    };
}

pub fn process_batch(accounts: &[AccountInfo], mut instruction_data: &[u8]) -> ProgramResult {
    const UNINIT_ACCOUNT: MaybeUninit<AccountInfo> = MaybeUninit::<AccountInfo>::uninit();
    // Instructions take at most 15 accounts.
    let mut instruction_accounts: [MaybeUninit<AccountInfo>; 15] = [UNINIT_ACCOUNT; 15];

    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    loop {
        let expected_accounts = unsafe { *instruction_data.get_unchecked(0) as usize };
        // There must be at least:
        //   - 1 byte for the number of accounts.
        //   - `expected_accounts` bytes for instruction accounts indices.
        //   - 1 byte for the length of the instruction data.
        let data_offset = expected_accounts + 2;

        if instruction_data.len() < data_offset {
            return Err(ProgramError::InvalidInstructionData);
        }

        let indices = unsafe { instruction_data.get_unchecked(1..1 + expected_accounts) };
        fill_accounts!(indices, accounts, instruction_accounts);

        let expected_data =
            data_offset + unsafe { *instruction_data.get_unchecked(data_offset - 1) as usize };

        if instruction_data.len() < expected_data || expected_data == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        // SAFETY: The instruction data and accounts lengths are already validated so all
        // the slices are guaranteed to be valid.
        inner_process_instruction(
            unsafe {
                core::slice::from_raw_parts(instruction_accounts.as_ptr() as _, expected_accounts)
            },
            unsafe { instruction_data.get_unchecked(data_offset + 1..expected_data) },
            unsafe { *instruction_data.get_unchecked(data_offset) },
        )?;

        if expected_data == instruction_data.len() {
            // The batch is complete.
            break;
        }

        instruction_data = &instruction_data[expected_data..];
    }

    Ok(())
}
