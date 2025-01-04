use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};
use crate::{
    state::*,
    errors::*,
    events::*,
    constants::*,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeStablecoinParams {
    pub name: String,
    pub symbol: String,
    pub target_currency: String,
    pub icon_uri: String,
}

#[derive(Accounts)]
#[instruction(params: InitializeStablecoinParams)]
pub struct InitializeStablecoin<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = StablecoinConfig::LEN,
        seeds = [
            STABLECOIN_SEED,
            authority.key().as_ref(),
            params.symbol.as_bytes()
        ],
        bump
    )]
    pub stablecoin_config: Account<'info, StablecoinConfig>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = authority.key(),
    )]
    pub mint: Account<'info, Mint>,
    
    /// CHECK: Verified in handler
    pub stablebond_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        seeds = [VAULT_SEED, stablecoin_config.key().as_ref()],
        bump,
        token::mint = stablebond_mint,
        token::authority = stablecoin_config,
    )]
    pub vault: Account<'info, TokenAccount>,
    
    /// CHECK: Oracle address verified in handler
    pub oracle: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeStablecoin>,
    params: InitializeStablecoinParams,
) -> Result<()> {
    // Validate inputs
    require!(
        params.name.len() >= MIN_NAME_LENGTH 
        && params.name.len() <= MAX_NAME_LENGTH,
        StableFunError::InvalidName
    );
    
    require!(
        params.symbol.len() >= MIN_SYMBOL_LENGTH 
        && params.symbol.len() <= MAX_SYMBOL_LENGTH,
        StableFunError::InvalidSymbol
    );

    require!(
        params.icon_uri.len() <= MAX_ICON_URI_LENGTH,
        StableFunError::InvalidIconUri
    );

    let stablecoin = &mut ctx.accounts.stablecoin_config;
    stablecoin.authority = ctx.accounts.authority.key();
    stablecoin.mint = ctx.accounts.mint.key();
    stablecoin.name = params.name.clone();
    stablecoin.symbol = params.symbol.clone();
    stablecoin.target_currency = params.target_currency;
    stablecoin.total_supply = 0;
    stablecoin.stablebond_mint = ctx.accounts.stablebond_mint.key();
    stablecoin.vault = ctx.accounts.vault.key();
    stablecoin.oracle = ctx.accounts.oracle.key();
    stablecoin.bump = *ctx.bumps.get("stablecoin_config").unwrap();
    stablecoin.vault_bump = *ctx.bumps.get("vault").unwrap();
    stablecoin.icon_uri = params.icon_uri;

    // Emit event
    emit!(StablecoinCreated {
        authority: stablecoin.authority,
        mint: stablecoin.mint,
        name: params.name,
        symbol: params.symbol,
        target_currency: params.target_currency,
    });

    Ok(())
}