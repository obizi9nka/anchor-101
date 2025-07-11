use anchor_spl::token_2022::spl_token_2022::instruction::mint_to;
use anchor_spl::token_2022::spl_token_2022;

use crate::*;
use crate::utils::{ create_new_token, create_associated_token_account };

pub const TITLE: &str = "Title";
pub const MESSAGE: &str = "init message";

#[test]
pub fn test_initialize_config() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let same_payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let token_mint = create_new_token(&client, &payer, 6).unwrap();

    let ata = create_associated_token_account(
        &client,
        &payer,
        &token_mint,
        &payer.pubkey()
    ).unwrap();

    let token_program = client.program(spl_token_2022::ID).unwrap();

    // Проверяем баланс до минтинга
    let token_account_data_before = token_program.rpc().get_token_account_balance(&ata).unwrap();
    println!("Balance before minting: {:?}", token_account_data_before);

    // Минтим токены на аккаунт (используем Token 2022 программу)
    let mint_amount = 1000 * (10_u64).pow(6); // 1000 токенов с 6 десятичными знаками
    let mint_to_ix = mint_to(
        &anchor_spl::token_2022::ID,
        &token_mint,
        &ata,
        &payer.pubkey(),
        &[],
        mint_amount
    ).unwrap();

    let transaction = anchor_client::solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&payer.pubkey()),
        &[&same_payer],
        token_program.rpc().get_latest_blockhash().unwrap()
    );

    let signature = token_program.rpc().send_and_confirm_transaction(&transaction).unwrap();
    println!("Mint transaction signature: {}", signature);

    // Проверяем баланс после минтинга
    let token_account_data_after = token_program.rpc().get_token_account_balance(&ata).unwrap();
    println!("Balance after minting: {:?}", token_account_data_after);

    assert_eq!(token_account_data_after.amount, mint_amount.to_string());

    let (config_pda, _) = Pubkey::find_program_address(
        &[b"lottery_config", payer.pubkey().as_ref()],
        &program_id
    );
}
