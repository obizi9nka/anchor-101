use crate::*;

pub const POLL_ID: u64 = 1;

#[test]
pub fn test_initialize_poll() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let (poll_pda, _bump) = Pubkey::find_program_address(
        &[POLL_ID.to_le_bytes().as_ref()],
        &program_id
    );

    let tx = program
        .request()
        .accounts(voting::accounts::InitializePoll {
            signer: payer.pubkey(),
            poll: poll_pda,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(voting::instruction::InitializePoll {
            poll_id: POLL_ID,
            poll_start: 1,
            poll_end: 1000000000000000000,
            description: "Test poll".to_string(),
        })
        .send()
        .expect("Failed to send transaction");

    println!("Your transaction signature {}", tx);
}
