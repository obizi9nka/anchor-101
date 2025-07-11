use anchor_client::anchor_lang::prelude::*;
use anchor_client::solana_sdk::{ signature::Keypair, signature::Signer, system_instruction };
use anchor_spl::token_2022::spl_token_2022::state::PackedSizeOf;
use anchor_spl::token_2022::{
    self,
    spl_token_2022::{ instruction::{ initialize_mint2 }, state::Mint },
};

pub fn create_new_token(client: &Client, payer: &Keypair, decimals: u8) -> Result<Pubkey> {
    // Создаем новый keypair для минта
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    // Вычисляем размер аккаунта для минта
    let mint_rent = rpc_client.get_minimum_balance_for_rent_exemption(Mint::SIZE_OF).unwrap();

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
    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let transaction = anchor_client::solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[payer, &mint_keypair],
        recent_blockhash
    );

    // Отправляем транзакцию
    let signature = rpc_client.send_and_confirm_transaction(&transaction).unwrap();

    println!("Token created with signature: {}", signature);
    println!("Token mint address: {}", mint_pubkey);

    Ok(mint_pubkey)
}

// Удобная функция для создания токена с стандартными параметрами
pub fn create_mock_token() -> Result<Pubkey> {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let cluster = Cluster::Localnet;

    // Создаем токен с 6 десятичными знаками (как у USDC)
    create_new_token(&rpc_client, &payer, 6)
}
