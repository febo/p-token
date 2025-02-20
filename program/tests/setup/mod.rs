use solana_sdk::pubkey::Pubkey;

#[allow(dead_code)]
pub mod account;
#[allow(dead_code)]
pub mod mint;

#[allow(dead_code)]
pub const TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array(token_interface::program::ID);
