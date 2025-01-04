use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use switchboard_v2::{AggregatorAccountData, SWITCHBOARD_PROGRAM_ID};
use crate::{state::*, errors::*, events::*, constants::*};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            STABLECOIN_SEED,
            stablecoin_config.authority.as_ref(),
            stablecoin_config.symbol.as_bytes()
        ],
        bump = stablecoin_config.bump
    )]
    pub stablecoin_config: Account<'info, StablecoinConfig>,
    
    #[account(
        mut,
        constraint = mint.key() == stablecoin_config.mint
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = user_token_account.mint == mint.key(),
        constraint = user_token_account.owner == user.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = user_stablebond_account.mint == stablecoin_config.stablebond_mint,
        constraint = user_stablebond_account.owner == user.key()
    )]
    pub user_stablebond_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [VAULT_SEED, stablecoin_config.key().as_ref()],
        bump = stablecoin_config.vault_bump
    )]
    pub vault: Account<'info, TokenAccount>,
    
    /// CHECK: Oracle account verified in handler
    #[account(
        constraint = oracle.owner == &SWITCHBOARD_PROGRAM_ID,
        constraint = oracle.key() == stablecoin_config.oracle
    )]
    pub oracle: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub switchboard_program: Program<'info, switchboard_v2::program::Switchboard>,
}

pub fn handler(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    require!(amount > 0, StableFunError::InvalidAmount);

    // Get oracle price
    let oracle_acc = ctx.accounts.oracle.to_account_info();
    let feed = AggregatorAccountData::new(&oracle_acc)
        .map_err(|_| StableFunError::InvalidOracleAccount)?;
    
    // Verify oracle is fresh
    let staleness_threshold = 300; // 5 minutes
    require!(
        feed.is_price_fresh(Clock::get()?.unix_timestamp, staleness_threshold),
        StableFunError::StaleOracle
    );

    // Get latest price with 9 decimals precision
    let oracle_price = feed.get_latest_value()
        .map_err(|_| StableFunError::InvalidOraclePrice)?;

    // Calculate required collateral amount with fees
    let base_collateral = amount
        .checked_mul(oracle_price)
        .ok_or(StableFunError::MathOverflow)?
        .checked_div(PRICE_DECIMALS)
        .ok_or(StableFunError::MathOverflow)?;

    // Apply collateralization ratio (e.g., 120% = 12000)
    let collateral_amount = base_collateral
        .checked_mul(ctx.accounts.stablecoin_config.collateral_ratio)
        .ok_or(StableFunError::MathOverflow)?
        .checked_div(10000)
        .ok_or(StableFunError::MathOverflow)?;

    // Calculate and add fees
    let fee_amount = base_collateral
        .checked_mul(ctx.accounts.stablecoin_config.fee_percentage)
        .ok_or(StableFunError::MathOverflow)?
        .checked_div(10000)
        .ok_or(StableFunError::MathOverflow)?;

    let total_collateral = collateral_amount
        .checked_add(fee_amount)
        .ok_or(StableFunError::MathOverflow)?;

    // Verify user has enough stablebonds
    require!(
        ctx.accounts.user_stablebond_account.amount >= total_collateral,
        StableFunError::InsufficientCollateral
    );

    // Transfer stablebonds from user to vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_stablebond_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        total_collateral,
    )?;

    // Mint stablecoins to user
    let mint_authority_seeds = &[
        STABLECOIN_SEED,
        ctx.accounts.stablecoin_config.authority.as_ref(),
        ctx.accounts.stablecoin_config.symbol.as_bytes(),
        &[ctx.accounts.stablecoin_config.bump],
    ];
    let mint_authority_signer = &[&mint_authority_seeds[..]];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.stablecoin_config.to_account_info(),
            },
            mint_authority_signer,
        ),
        amount,
    )?;

    // Update total supply
    ctx.accounts.stablecoin_config.total_supply = ctx
        .accounts
        .stablecoin_config
        .total_supply
        .checked_add(amount)
        .ok_or(StableFunError::MathOverflow)?;

    // Emit event
    emit!(TokensMinted {
        mint: ctx.accounts.mint.key(),
        user: ctx.accounts.user.key(),
        amount,
        collateral_amount: total_collateral,
    });

    Ok(())
}