use crate::*;

pub const TITLE: &str = "Title";
pub const MESSAGE: &str = "init message";

#[test]
pub fn test_initialize_config() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    let program = client.program(program_id).unwrap();

    let (config_pda, _) = Pubkey::find_program_address(
        &[b"lottery_config", payer.pubkey().as_ref()],
        &program_id
    );
}
