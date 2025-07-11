use token_lottery::FEE_PRECESSION;

use crate::*;

#[test]
pub fn test_initialize_config() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let token_mint = create_new_token(&client, &payer, 6).unwrap();

    let (config_pda, _) = Pubkey::find_program_address(
        &[b"lottery_config", payer.pubkey().as_ref()],
        &program_id
    );

    let (treasury_pda, _) = Pubkey::find_program_address(
        &[b"lottery_treasury", payer.pubkey().as_ref()],
        &program_id
    );

    program
        .request()
        .accounts(InitializeConfig {
            signer: payer.pubkey(),
            config: config_pda,
            mint: token_mint,
            treasury_lottery_token_account: treasury_pda,
            system_program: anchor_client::solana_sdk::system_program::ID,
            token_program: spl_token_2022::ID,
        })
        .args(token_lottery::instruction::InitializeConfig {
            ticket_price: 100 * (10_u64).pow(6),
            fee: 100,
        })
        .send()
        .unwrap();

    let config = program.account::<Config>(config_pda).unwrap();
    println!("Config: {:?}", config.ticket_price);
    println!("Config: {:?}", config.fee);
    println!("Config: {:?}", config.token);

    assert_eq!(config.ticket_price, 100 * (10_u64).pow(6));
    assert_eq!(config.fee, 100);
    assert_eq!(config.token, token_mint);
}

#[test]
pub fn test_initialize_config_revert_if_fee_is_too_high() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let token_mint = create_new_token(&client, &payer, 6).unwrap();

    let (config_pda, _) = Pubkey::find_program_address(
        &[b"lottery_config", payer.pubkey().as_ref()],
        &program_id
    );

    let (treasury_pda, _) = Pubkey::find_program_address(
        &[b"lottery_treasury", payer.pubkey().as_ref()],
        &program_id
    );

    program
        .request()
        .accounts(InitializeConfig {
            signer: payer.pubkey(),
            config: config_pda,
            mint: token_mint,
            treasury_lottery_token_account: treasury_pda,
            system_program: anchor_client::solana_sdk::system_program::ID,
            token_program: spl_token_2022::ID,
        })
        .args(token_lottery::instruction::InitializeConfig {
            ticket_price: 100 * (10_u64).pow(6),
            fee: *FEE_PRECESSION + 1,
        })
        .send()
        .expect_err("Should revert due to fee is too high");
}
