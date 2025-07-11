pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("dkeKcB81o8nCGzuGQbB7woAMpQPYV4sJE1Q3RwvBPYq");

#[program]
pub mod token_lottery {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        ticket_price: u64,
        fee: u16
    ) -> Result<()> {
        initialize_config::handler(ctx, ticket_price, fee)
    }

    //     pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {}

    //     pub fn start_lottery_draw(ctx: Context<StartLotteryDraw>) -> Result<()> {}

    //     pub fn finish_lottery_draw(ctx: Context<FinishLotteryDraw>) -> Result<()> {}
}
