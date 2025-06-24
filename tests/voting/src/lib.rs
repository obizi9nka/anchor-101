#[cfg(test)]
mod test_initialize_candidate;
mod test_initialize_poll;
mod test_vote;

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

pub const PROGRAM_ID: &str = "2BbtcVmvELEW3kz3F8vDVHV4cRkP24BaSKpU9hVfdHWL";
pub const POLL_ID: u64 = 1;
