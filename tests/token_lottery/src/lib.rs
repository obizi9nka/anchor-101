#[cfg(test)]
mod test_initialize_config;
mod utils;

use anchor_client::anchor_lang::prelude::*;
use anchor_client::anchor_lang::solana_program::example_mocks::solana_sdk;

use anchor_spl::token_interface;

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

pub const PROGRAM_ID: &str = "dkeKcB81o8nCGzuGQbB7woAMpQPYV4sJE1Q3RwvBPYq";
