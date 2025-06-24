use crate::*;

#[test]
pub fn test_update_journal_entry() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let (journal_pda, _) = Pubkey::find_program_address(
        &[test_create_journal_entry::TITLE.as_bytes().as_ref(), payer.pubkey().as_ref()],
        &program_id
    );

    test_create_journal_entry::test_create_journal_entry();

    let mut journal = program.account::<crud::JournalEntryState>(journal_pda).unwrap();

    assert_eq!(journal.title, test_create_journal_entry::TITLE);

    let update_message = "update message";

    let _ = program
        .request()
        .accounts(crud::accounts::UpdateJournalEntry {
            owner: payer.pubkey(),
            journal_entry: journal_pda,
            system_program: anchor_client::solana_sdk::system_program::ID,
        })
        .args(crud::instruction::Update {
            _title: test_create_journal_entry::TITLE.to_string(),
            message: update_message.to_string(),
        })
        .send();

    journal = program.account::<crud::JournalEntryState>(journal_pda).unwrap();

    assert_eq!(journal.message, update_message);
}
