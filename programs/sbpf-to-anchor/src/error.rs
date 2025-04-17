// Custom error codes
#[error_code]
pub enum SwapError {
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid DEX type")]
    InvalidDexType,
    #[msg("Invalid pool state")]
    InvalidPoolState,
    #[msg("Insufficient balance")]
    InsufficientBalance,
    #[msg("Invalid global controller")]
    InvalidGlobalController,
    #[msg("Invalid swap direction")]
    InvalidSwapDirection,
    #[msg("Invalid quote")]
    InvalidQuote,
    #[msg("Invalid validator")]
    InvalidValidator,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic error occurred")]
    ArithmeticError,
    #[msg("Sandwich trade already complete")]
    SandwichAlreadyComplete,
    #[msg("Invalid parameter provided")]
    InvalidParameter,
    #[msg("Arithmetic overflow occurred")]
    Overflow,
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Price out of range")]
    PriceOutOfRange,
    #[msg("Invalid quote")]
    InvalidQuote,
    #[msg("Invalid validator")]
    InvalidValidator,
}
