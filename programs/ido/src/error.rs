use anchor_lang::prelude::error_code;
#[error_code]
pub enum ErrorCode {
    #[msg("IDO must start in the future")]
    IdoFuture,
    #[msg("IDO length must be longer than 1 default round length")]
    IdoShort,
    #[msg("IDO times are non-sequential")]
    SeqTimes,
    // #[msg("IDO has not started")]
    // StartIdoTime,
    // #[msg("Deposits period has ended")]
    // EndDepositsTime,
    // #[msg("IDO has ended")]
    // EndIdoTime,
    // #[msg("IDO has not finished yet")]
    // IdoNotOver,
    // #[msg("Escrow period has not finished yet")]
    // EscrowNotOver,
    // #[msg("Insufficient USDC")]
    // LowUsdc,
    // #[msg("Insufficient redeemable tokens")]
    // LowRedeemable,
    // #[msg("USDC total and redeemable total don't match")]
    // UsdcNotEqRedeem,
    // #[msg("Given nonce is invalid")]
    // InvalidNonce,
}
