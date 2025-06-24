use crate::*;

#[test]
pub fn test_vote() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    test_initialize_candidate::test_initialize_candidate();

    let (poll_pda, _bump) = Pubkey::find_program_address(
        &[POLL_ID.to_le_bytes().as_ref()],
        &program_id
    );

    let (candidate_pda, _bump) = Pubkey::find_program_address(
        &[test_initialize_candidate::CANDIDATE_NAME.as_bytes().as_ref()],
        &program_id
    );

    let _ = program
        .request()
        .accounts(voting::accounts::Vote {
            signer: payer.pubkey(),
            poll: poll_pda,
            candidate: candidate_pda,
        })
        .args(voting::instruction::Vote {
            _candidate_name: test_initialize_candidate::CANDIDATE_NAME.to_string(),
            _poll_id: POLL_ID,
        })
        .send()
        .expect("Failed to send transaction");

    let candidate = program.account::<voting::Candidate>(candidate_pda).unwrap();
    assert_eq!(candidate.candidate_votes, 1);
}
