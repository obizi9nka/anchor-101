use crate::*;

#[test]
pub fn test_create_mock_token() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());

    let token_mint = create_new_token(&client, &payer, 6).unwrap();

    let ata = create_associated_token_account(
        &client,
        &payer,
        &token_mint,
        &payer.pubkey()
    ).unwrap();

    let token_program = client.program(spl_token_2022::ID).unwrap();

    let token_account_data_before = token_program.rpc().get_token_account_balance(&ata).unwrap();
    println!("Balance before minting: {:?}", token_account_data_before);

    let mint_amount = 1000 * (10_u64).pow(6);
    mint_tokens(&client, &payer, &token_mint, &ata, mint_amount).unwrap();

    let token_account_data_after = token_program.rpc().get_token_account_balance(&ata).unwrap();
    println!("Balance after minting: {:?}", token_account_data_after);

    assert_eq!(token_account_data_after.amount, mint_amount.to_string());
}
