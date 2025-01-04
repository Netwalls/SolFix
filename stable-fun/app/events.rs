use anchor_lang::prelude::*;

#[event]
pub struct StablecoinCreated {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub target_currency: String,
}

#[event]
pub struct TokensMinted {
    pub mint: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub collateral_amount: u64,
}

#[event]
pub struct TokensRedeemed {
    pub mint: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub collateral_returned: u64,
}
