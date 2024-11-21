use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::processor::{
    amount_to_ui_amount::process_amount_to_ui_amount, approve::process_approve,
    approve_checked::process_approve_checked, burn::process_burn,
    burn_checked::process_burn_checked, close_account::process_close_account,
    freeze_account::process_freeze_account, get_account_data_size::process_get_account_data_size,
    initialize_account::process_initialize_account,
    initialize_account2::process_initialize_account2,
    initialize_account3::process_initialize_account3,
    initialize_immutable_owner::process_initialize_immutable_owner,
    initialize_mint::process_initialize_mint, initialize_mint2::process_initialize_mint2,
    initialize_multisig::process_initialize_multisig,
    initialize_multisig2::process_initialize_multisig2, mint_to::process_mint_to,
    mint_to_checked::process_mint_to_checked, revoke::process_revoke,
    set_authority::process_set_authority, sync_native::process_sync_native,
    thaw_account::process_thaw_account, transfer::process_transfer,
    transfer_checked::process_transfer_checked, ui_amount_to_amount::process_ui_amount_to_amount,
};

entrypoint!(process_instruction);

#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        // 0 - InitializeMint
        Some((&0, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMint");

            process_initialize_mint(program_id, accounts, data)
        }
        // 1 - InitializeAccount
        Some((&1, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeAccount");

            process_initialize_account(program_id, accounts, data)
        }
        // 2 - InitializeMultisig
        Some((&2, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMultisig");

            process_initialize_multisig(program_id, accounts, data)
        }
        // 3 - Transfer
        Some((&3, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Transfer");

            process_transfer(program_id, accounts, data)
        }
        // 4 - Approve
        Some((&4, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Approve");

            process_approve(program_id, accounts, data)
        }
        // 5 - Revoke
        Some((&5, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Revoke");

            process_revoke(program_id, accounts, data)
        }
        // 6 - SetAuthority
        Some((&6, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: SetAuthority");

            process_set_authority(program_id, accounts, data)
        }
        // 7 - MintTo
        Some((&7, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: MintTo");

            process_mint_to(program_id, accounts, data)
        }
        // 8 - Burn
        Some((&8, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Burn");

            process_burn(program_id, accounts, data)
        }
        // 9 - CloseAccount
        Some((&9, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: CloseAccount");

            process_close_account(program_id, accounts, data)
        }
        // 10 - FreezeAccount
        Some((&10, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: FreezeAccount");

            process_freeze_account(program_id, accounts, data)
        }
        // 11 - ThawAccount
        Some((&11, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: ThawAccount");

            process_thaw_account(program_id, accounts, data)
        }
        // 12 - TransferChecked
        Some((&12, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: TransferChecked");

            process_transfer_checked(program_id, accounts, data)
        }
        // 13 - ApproveChecked
        Some((&13, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: ApproveChecked");

            process_approve_checked(program_id, accounts, data)
        }
        // 14 - MintToChecked
        Some((&14, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: MintToChecked");

            process_mint_to_checked(program_id, accounts, data)
        }
        // 15 - BurnChecked
        Some((&15, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: BurnChecked");

            process_burn_checked(program_id, accounts, data)
        }
        // 16 - InitializeAccount2
        Some((&16, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeAccount2");

            process_initialize_account2(program_id, accounts, data)
        }
        // 17 - SyncNative
        Some((&17, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: SyncNative");

            process_sync_native(program_id, accounts, data)
        }
        // 18 - InitializeAccount3
        Some((&18, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeAccount3");

            process_initialize_account3(program_id, accounts, data)
        }
        // 19 - InitializeMultisig2
        Some((&19, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMultisig2");

            process_initialize_multisig2(program_id, accounts, data)
        }
        // 20 - InitializeMint2
        Some((&20, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMint2");

            process_initialize_mint2(program_id, accounts, data)
        }
        // 21 - GetAccountDataSize
        Some((&21, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: GetAccountDataSize");

            process_get_account_data_size(program_id, accounts, data)
        }
        // 22 - InitializeImmutableOwner
        Some((&22, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeImmutableOwner");

            process_initialize_immutable_owner(program_id, accounts, data)
        }
        // 23 - AmountToUiAmount
        Some((&23, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: AmountToUiAmount");

            process_amount_to_ui_amount(program_id, accounts, data)
        }
        // 24 - UiAmountToAmount
        Some((&24, data)) => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: UiAmountToAmount");

            process_ui_amount_to_amount(program_id, accounts, data)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
