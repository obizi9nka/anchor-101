use crate::*;

pub const TITLE: &str = "Title";
pub const MESSAGE: &str = "init message";

#[test]
pub fn test_create_journal_entry() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let (journal_pda, _) = Pubkey::find_program_address(
        &[TITLE.as_bytes().as_ref(), payer.pubkey().as_ref()],
        &program_id
    );

    program
        .request()
        .accounts(crud::accounts::CreateJournalEntry {
            owner: payer.pubkey(),
            journal_entry: journal_pda,
            system_program: solana_sdk::system_program::ID,
        })
        .args(crud::instruction::CreateJournalEntry {
            title: TITLE.to_string(),
            message: MESSAGE.to_string(),
        })
        .send()
        .unwrap();

    println!("journal_pda: {:?}", journal_pda);

    let journal = program.account::<crud::JournalEntryState>(journal_pda).unwrap();

    println!("journal owner: {:?}", journal.owner);

    assert_eq!(journal.title, TITLE);
    assert_eq!(journal.message, MESSAGE);
}
