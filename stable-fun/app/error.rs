use anchor_lang::prelude::*;

#[error_code]
pub enum StableFunError {
    #[msg("Invalid stablecoin name")]
    InvalidName,
    
    #[msg("Invalid symbol")]
    InvalidSymbol,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
    
    #[msg("Invalid oracle price")]
    InvalidOraclePrice,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Invalid oracle account")]
    InvalidOracleAccount,
    
    #[msg("Oracle price is stale")]
    StaleOracle,
    
    #[msg("Price below minimum")]
    PriceBelowMinimum,
    
    #[msg("Collateral ratio too low")]
    CollateralRatioTooLow,
    
    #[msg("Invalid icon URI")]
    InvalidIconUri,
}
