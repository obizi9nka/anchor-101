#[cfg(test)]
mod test_create_journal_entry;
mod test_update_journal_entry;
mod test_delete_journal_entry;

use anchor_client::anchor_lang::solana_program::example_mocks::solana_sdk;

use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::read_keypair_file,
        signer::Signer,
    },
    Client,
    Cluster,
};

pub const PROGRAM_ID: &str = "yUtKrVdS7gUsKwtKbX4JqKZCka9PmEdUVeubCH9bTz9";
