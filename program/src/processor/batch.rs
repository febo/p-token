use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::processor::{
    process_close_account, process_initialize_account3, process_initialize_mint,
    process_initialize_mint2, process_mint_to, process_transfer,
};

macro_rules! map_accounts {
    // For 1 account
    ($accounts:expr, $instruction_data:expr, 1) => {{
        let (account_idx, rest) = $instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        *$instruction_data = rest;
        let batch_accounts = [$accounts[*account_idx as usize].clone()];
        batch_accounts
    }};

    // For 2 accounts
    ($accounts:expr, $instruction_data:expr, 2) => {{
        let (account_indices, rest) = $instruction_data.split_at(2);
        *$instruction_data = rest;
        let batch_accounts = [
            $accounts[account_indices[0] as usize].clone(),
            $accounts[account_indices[1] as usize].clone(),
        ];
        batch_accounts
    }};

    // For 3 accounts
    ($accounts:expr, $instruction_data:expr, 3) => {{
        let (account_indices, rest) = $instruction_data.split_at(3);
        *$instruction_data = rest;
        let batch_accounts = [
            $accounts[account_indices[0] as usize].clone(),
            $accounts[account_indices[1] as usize].clone(),
            $accounts[account_indices[2] as usize].clone(),
        ];
        batch_accounts
    }};
}

#[inline(always)]
pub fn process_batch(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Validates the instruction data.
    let (counter, mut instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let mut discriminator;
    for _ in 0..*counter {
        (discriminator, instruction_data) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        match discriminator {
            // 0 - InitializeMint
            0 => {
                #[cfg(feature = "logging")]
                pinocchio::msg!("Batch Instruction: InitializeMint");

                let batch_accounts = map_accounts!(accounts, &mut instruction_data, 2);
                process_initialize_mint(&batch_accounts, instruction_data, true)?;
                if instruction_data[size_of::<(u8, Pubkey)>()] == 0 {
                    instruction_data = &instruction_data[size_of::<(u8, Pubkey, u8)>()..];
                } else {
                    instruction_data = &instruction_data[size_of::<(u8, Pubkey, u8, Pubkey)>()..];
                }
            }
            // 3 - Transfer
            3 => {
                #[cfg(feature = "logging")]
                pinocchio::msg!("Batch Instruction: Transfer");

                let batch_accounts = map_accounts!(accounts, &mut instruction_data, 3);
                process_transfer(&batch_accounts, instruction_data)?;
                instruction_data = &instruction_data[size_of::<u64>()..];
            }
            // 7 - MintTo
            7 => {
                #[cfg(feature = "logging")]
                pinocchio::msg!("Batch Instruction: MintTo");

                let batch_accounts = map_accounts!(accounts, &mut instruction_data, 3);
                process_mint_to(&batch_accounts, instruction_data)?;
                instruction_data = &instruction_data[size_of::<u64>()..];
            }
            // 9 - CloseAccount
            9 => {
                #[cfg(feature = "logging")]
                pinocchio::msg!("Batch Instruction: CloseAccount");

                let batch_accounts = map_accounts!(accounts, &mut instruction_data, 2);
                process_close_account(&batch_accounts)?;
            }
            18 => {
                #[cfg(feature = "logging")]
                pinocchio::msg!("Batch Instruction: InitializeAccount3");

                let batch_accounts = map_accounts!(accounts, &mut instruction_data, 3);
                process_initialize_account3(&batch_accounts, instruction_data)?;
                instruction_data = &instruction_data[size_of::<Pubkey>()..];
            }
            // 20 - InitializeMint2
            20 => {
                #[cfg(feature = "logging")]
                pinocchio::msg!("Instruction: InitializeMint2");

                let batch_accounts = map_accounts!(accounts, &mut instruction_data, 1);
                process_initialize_mint2(&batch_accounts, instruction_data)?;
                if instruction_data[size_of::<(u8, Pubkey)>()] == 0 {
                    instruction_data = &instruction_data[size_of::<(u8, Pubkey, u8)>()..];
                } else {
                    instruction_data = &instruction_data[size_of::<(u8, Pubkey, u8, Pubkey)>()..];
                }
            }
            _ => {
                return Err(ProgramError::InvalidInstructionData);
            }
        }
    }
    Ok(())
}
