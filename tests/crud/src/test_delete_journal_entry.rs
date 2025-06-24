use crate::*;

#[test]
pub fn test_delete_journal_entry() {
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

    test_update_journal_entry::test_update_journal_entry();

    // msg!("Title {}", program.account::<crud::JournalEntryState>(journal_pda).unwrap().title);

    let _ = program
        .request()
        .accounts(crud::accounts::DeleteJournalEntry {
            journal_entry: journal_pda,
            owner: payer.pubkey(),
            system_program: solana_sdk::system_program::ID,
        })
        .args(crud::instruction::Delete {
            _title: program.account::<crud::JournalEntryState>(journal_pda).unwrap().title,
        })
        .send();
}
