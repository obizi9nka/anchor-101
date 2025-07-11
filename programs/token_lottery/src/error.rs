use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Fee should be less than FEE_PRECESSION / 2")]
    MaxFeeExceed,
}
