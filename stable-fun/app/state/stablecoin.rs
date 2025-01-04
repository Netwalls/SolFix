use anchor_lang::prelude::*;
use switchboard_v2::{AggregatorAccountData, SWITCHBOARD_PROGRAM_ID};

#[account]
pub struct StablecoinConfig {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub target_currency: String,
    pub icon_uri: String,
    pub total_supply: u64,
    pub stablebond_mint: Pubkey,
    pub vault: Pubkey,
    pub oracle: Pubkey,
    pub collateral_ratio: u64,
    pub fee_percentage: u64,
    pub bump: u8,
    pub vault_bump: u8,
}

impl StablecoinConfig {
    pub const LEN: usize = 8 +    // discriminator
        32 +    // authority
        32 +    // mint
        64 +    // name
        32 +    // symbol
        32 +    // target_currency
        64 +    // icon_uri
        8 +     // total_supply
        32 +    // stablebond_mint
        32 +    // vault
        32 +    // oracle
        8 +     // collateral_ratio
        8 +     // fee_percentage
        1 +     // bump
        1;      // vault_bump
}
