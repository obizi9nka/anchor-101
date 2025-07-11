use anchor_client::anchor_lang::prelude::*;
use anchor_client::anchor_lang::solana_program::example_mocks::solana_sdk;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::{ Client, Cluster };
use anchor_client::solana_sdk::{ signature::Keypair, signature::Signer, system_instruction };
use anchor_spl::token::spl_token;
use anchor_spl::token_2022::spl_token_2022;
use anchor_spl::token_2022::spl_token_2022::state::PackedSizeOf;
use anchor_spl::token_2022::{
    self,
    spl_token_2022::{ instruction::{ initialize_mint2, mint_to }, state::Mint },
};
use anchor_spl::associated_token::get_associated_token_address_with_program_id;
use anchor_spl::associated_token;
use anchor_client::solana_sdk::instruction::Instruction;

pub fn create_new_token(
    client: &Client<&Keypair>,
    payer: &Keypair,
    decimals: u8
) -> Result<Pubkey> {
    // Создаем новый keypair для минта
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    let token_program = client.program(spl_token_2022::ID).unwrap();

    // Вычисляем размер аккаунта для минта
    let mint_rent = token_program
        .rpc()
        .get_minimum_balance_for_rent_exemption(Mint::SIZE_OF)
        .unwrap();

    // Создаем аккаунт для минта
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_pubkey,
        mint_rent,
        Mint::SIZE_OF as u64,
        &anchor_spl::token_2022::ID
    );

    // Инициализируем минт
    let initialize_mint_ix = initialize_mint2(
        &anchor_spl::token_2022::ID,
        &mint_pubkey,
        &payer.pubkey(), // mint_authority
        Some(&payer.pubkey()), // freeze_authority
        decimals
    ).unwrap();

    // Создаем транзакцию
    let recent_blockhash = token_program.rpc().get_latest_blockhash().unwrap();
    let transaction = anchor_client::solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[payer, &mint_keypair],
        recent_blockhash
    );

    // Отправляем транзакцию
    let signature = token_program.rpc().send_and_confirm_transaction(&transaction).unwrap();

    println!("Token created with signature: {}", signature);
    println!("Token mint address: {}", mint_pubkey);

    Ok(mint_pubkey)
}

pub fn create_associated_token_account(
    client: &Client<&Keypair>,
    payer: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey
) -> Result<Pubkey> {
    // Вычисляем адрес ATA
    let ata_address = get_associated_token_address_with_program_id(
        owner,
        mint,
        &anchor_spl::token_2022::ID
    );

    // Создаем инструкцию для создания ATA
    let create_ata_ix = Instruction {
        program_id: anchor_spl::associated_token::ID,
        accounts: vec![
            anchor_client::solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
            anchor_client::solana_sdk::instruction::AccountMeta::new(ata_address, false),
            anchor_client::solana_sdk::instruction::AccountMeta::new_readonly(*owner, false),
            anchor_client::solana_sdk::instruction::AccountMeta::new_readonly(*mint, false),
            anchor_client::solana_sdk::instruction::AccountMeta::new_readonly(
                anchor_client::solana_sdk::system_program::ID,
                false
            ),
            anchor_client::solana_sdk::instruction::AccountMeta::new_readonly(
                anchor_spl::token_2022::ID,
                false
            )
        ],
        data: vec![],
    };

    // Создаем и отправляем транзакцию
    let recent_blockhash = client
        .program(anchor_spl::token_2022::ID)
        .unwrap()
        .rpc()
        .get_latest_blockhash()
        .unwrap();
    let transaction = anchor_client::solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash
    );

    let signature = client
        .program(anchor_spl::token_2022::ID)
        .unwrap()
        .rpc()
        .send_and_confirm_transaction(&transaction)
        .unwrap();

    println!("ATA created with signature: {}", signature);
    println!("ATA address: {}", ata_address);

    Ok(ata_address)
}

// Удобная функция для создания токена с стандартными параметрами
pub fn create_mock_token() -> Result<Pubkey> {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());

    // Создаем токен с 6 десятичными знаками (как у USDC)
    create_new_token(&client, &payer, 6)
}
