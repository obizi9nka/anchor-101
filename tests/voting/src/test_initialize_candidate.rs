use crate::*;

pub const CANDIDATE_NAME: &str = "Candidate 1";

#[test]
pub fn test_initialize_candidate() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let (poll_pda, _bump) = Pubkey::find_program_address(
        &[test_initialize_poll::POLL_ID.to_le_bytes().as_ref()],
        &program_id
    );

    test_initialize_poll::test_initialize_poll();

    let (candidate_pda, _bump) = Pubkey::find_program_address(
        &[CANDIDATE_NAME.as_bytes().as_ref()],
        &program_id
    );

    let candidate_tx = program
        .request()
        .accounts(voting::accounts::InitializeCandidate {
            signer: payer.pubkey(),
            poll: poll_pda,
            candidate: candidate_pda,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(voting::instruction::InitializeCandidate {
            candidate_name: CANDIDATE_NAME.to_string(),
            _poll_id: test_initialize_poll::POLL_ID,
        })
        .send()
        .expect("Failed to send transaction");

    println!("Candidate created with signature: {}", candidate_tx);
}
