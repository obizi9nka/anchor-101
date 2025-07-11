#[cfg(test)]
mod test_initialize_config;
#[cfg(test)]
mod test_create_mock_token;

mod utils;

use utils::*;

use anchor_client::anchor_lang::prelude::*;
use anchor_client::anchor_lang::solana_program::example_mocks::solana_sdk;

use anchor_spl::token_interface::spl_token_2022;

use token_lottery::{ accounts::InitializeConfig, Config };

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

pub const PROGRAM_ID: &str = "BWPD475AAzWqfTJfJDCEzYNkPBWizQY3r4avUCdnVt4C";
