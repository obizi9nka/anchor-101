use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Lottery {
    pub pot: u64,
    pub start_time: i64,
    pub finish_time: i64,
    pub request_randomness_time: i64,
    #[max_len(1000)]
    pub participants: Vec<Pubkey>,
    pub randomness: u64,
}
