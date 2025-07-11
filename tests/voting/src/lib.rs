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

pub const PROGRAM_ID: &str = "9VN6q6U4duLcSfZNFFF7yKPzqg1qRQe6EKuHUEhPv76v";
pub const POLL_ID: u64 = 1;
