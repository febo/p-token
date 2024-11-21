#![cfg(feature = "test-sbf")]

mod setup;

use setup::{account, mint};
use solana_program_test::{tokio, ProgramTest};
use solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[test_case::test_case(spl_token::ID ; "spl-token")]
#[test_case::test_case(Pubkey::new_from_array(token_program::ID) ; "p-token")]
#[tokio::test]
async fn mint_to_checked(token_program: Pubkey) {
    let program_id = Pubkey::new_from_array(token_program::ID);
    let mut context = ProgramTest::new("token_program", program_id, None)
        .start_with_context()
        .await;

    // Given a mint account.

    let mint_authority = Keypair::new();
    let freeze_authority = Pubkey::new_unique();

    let mint = mint::initialize(
        &mut context,
        mint_authority.pubkey(),
        Some(freeze_authority),
        &token_program,
    )
    .await
    .unwrap();

    // And a token account.

    let owner = Keypair::new();

    let account = account::initialize(&mut context, &mint, &owner.pubkey(), &token_program).await;

    // When we mint tokens to it.

    let mut mint_ix = spl_token::instruction::mint_to_checked(
        &spl_token::ID,
        &mint,
        &account,
        &mint_authority.pubkey(),
        &[],
        100,
        0,
    )
    .unwrap();
    // Switches the program id to the token program.
    mint_ix.program_id = token_program;

    let tx = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &mint_authority],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Then an account has the correct data.

    let account = context.banks_client.get_account(account).await.unwrap();

    assert!(account.is_some());

    let account = account.unwrap();
    let account = spl_token::state::Account::unpack(&account.data).unwrap();

    assert!(account.amount == 100);
}