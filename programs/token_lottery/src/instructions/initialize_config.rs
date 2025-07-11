use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface };
use crate::{ error::ErrorCode, state::Config, FEE_PRECESSION };

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space = 8 + Config::INIT_SPACE,
        payer = signer,
        seeds = [b"lottery_config", signer.key().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        token::mint = mint,
        token::authority = treasury_lottery_token_account,
        payer = signer,
        seeds = [b"lottery_treasury", signer.key().as_ref()],
        bump
    )]
    pub treasury_lottery_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<InitializeConfig>, ticket_price: u64, fee: u16) -> Result<()> {
    if fee > FEE_PRECESSION / 2 {
        return Err(ErrorCode::MaxFeeExceed.into());
    }

    *ctx.accounts.config = Config {
        token: ctx.accounts.mint.key(),
        ticket_price,
        fee,
    };

    Ok(())
}
