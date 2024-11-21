use core::marker::PhantomData;

use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use token_interface::{
    error::TokenError,
    instruction::AuthorityType,
    state::{account::Account, mint::Mint, PodCOption},
};

use super::validate_owner;

#[inline(never)]
pub fn process_set_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let args = SetAuthority::try_from_bytes(instruction_data)?;
    let authority_type = args.authority_type();
    let new_authority = args.new_authority();

    let [account_info, authority_info, remaning @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if account_info.data_len() == Account::LEN {
        let account = bytemuck::try_from_bytes_mut::<Account>(unsafe {
            account_info.borrow_mut_data_unchecked()
        })
        .map_err(|_error| ProgramError::InvalidAccountData)?;

        if account.is_frozen() {
            return Err(TokenError::AccountFrozen.into());
        }

        match authority_type {
            AuthorityType::AccountOwner => {
                validate_owner(program_id, &account.owner, authority_info, remaning)?;

                if let Some(authority) = new_authority {
                    account.owner = *authority;
                } else {
                    return Err(TokenError::InvalidInstruction.into());
                }

                account.delegate.clear();
                account.delegated_amount = 0.into();

                if account.is_native.is_some() {
                    account.close_authority.clear();
                }
            }
            AuthorityType::CloseAccount => {
                let authority = account.close_authority.as_ref().unwrap_or(&account.owner);
                validate_owner(program_id, authority, authority_info, remaning)?;
                account.close_authority = PodCOption::from(new_authority.copied());
            }
            _ => {
                return Err(TokenError::AuthorityTypeNotSupported.into());
            }
        }
    } else if account_info.data_len() == Mint::LEN {
        let mint = bytemuck::try_from_bytes_mut::<Mint>(unsafe {
            account_info.borrow_mut_data_unchecked()
        })
        .map_err(|_error| ProgramError::InvalidAccountData)?;

        match authority_type {
            AuthorityType::MintTokens => {
                // Once a mint's supply is fixed, it cannot be undone by setting a new
                // mint_authority
                let mint_authority = mint
                    .mint_authority
                    .as_ref()
                    .ok_or(TokenError::FixedSupply)?;

                validate_owner(program_id, mint_authority, authority_info, remaning)?;
                mint.mint_authority = PodCOption::from(new_authority.copied());
            }
            AuthorityType::FreezeAccount => {
                // Once a mint's freeze authority is disabled, it cannot be re-enabled by
                // setting a new freeze_authority
                let freeze_authority = mint
                    .freeze_authority
                    .as_ref()
                    .ok_or(TokenError::MintCannotFreeze)?;

                validate_owner(program_id, freeze_authority, authority_info, remaning)?;
                mint.freeze_authority = PodCOption::from(new_authority.copied());
            }
            _ => {
                return Err(TokenError::AuthorityTypeNotSupported.into());
            }
        }
    } else {
        return Err(ProgramError::InvalidArgument);
    }

    Ok(())
}

struct SetAuthority<'a> {
    raw: *const u8,

    _data: PhantomData<&'a [u8]>,
}

impl SetAuthority<'_> {
    #[inline(always)]
    pub fn try_from_bytes(bytes: &[u8]) -> Result<SetAuthority, ProgramError> {
        // The minimum expected size of the instruction data.
        // - authority_type (1 byte)
        // - option + new_authority (1 byte + 32 bytes)
        if bytes.len() < 2 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(SetAuthority {
            raw: bytes.as_ptr(),
            _data: PhantomData,
        })
    }

    #[inline(always)]
    pub fn authority_type(&self) -> AuthorityType {
        unsafe { AuthorityType::from(*self.raw) }
    }

    #[inline(always)]
    pub fn new_authority(&self) -> Option<&Pubkey> {
        unsafe {
            if *self.raw.add(33) == 0 {
                Option::None
            } else {
                Option::Some(&*(self.raw.add(34) as *const Pubkey))
            }
        }
    }
}
