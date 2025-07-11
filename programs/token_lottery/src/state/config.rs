use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub token: Pubkey,
    pub ticket_price: u64,
    pub fee: u16,
}
