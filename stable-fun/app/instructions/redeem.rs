use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{state::*, errors::*, events::*, constants::*};
use switchboard_v2::{AggregatorAccountData, SWITCHBOARD_PROGRAM_ID};


#[derive(Accounts)]
pub struct RedeemTokens<'info> {
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
    #[account(constraint = oracle.key() == stablecoin_config.oracle)]
    pub oracle: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RedeemTokens>, amount: u64) -> Result<()> {
    require!(amount > 0, StableFunError::InvalidAmount);
    
    // Verify user has enough tokens
    require!(
        ctx.accounts.user_token_account.amount >= amount,
        StableFunError::InsufficientCollateral
    );

    // Get oracle price and calculate exchange rate
    // This is a placeholder - you'll need to implement actual oracle integration
    let exchange_rate = 1; // 1:1 for example

    let collateral_amount = amount
        .checked_mul(exchange_rate)
        .ok_or(StableFunError::MathOverflow)?;

    // Verify vault has enough collateral
    require!(
        ctx.accounts.vault.amount >= collateral_amount,
        StableFunError::InsufficientCollateral
    );

    // Burn stablecoins from user
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    // Transfer stablebonds from vault to user
    let seeds = &[
        STABLECOIN_SEED,
        ctx.accounts.stablecoin_config.authority.as_ref(),
        ctx.accounts.stablecoin_config.symbol.as_bytes(),
        &[ctx.accounts.stablecoin_config.bump],
    ];
    let signer = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user_stablebond_account.to_account_info(),
                authority: ctx.accounts.stablecoin_config.to_account_info(),
            },
            signer,
        ),
        collateral_amount,
    )?;

    // Update total supply
    ctx.accounts.stablecoin_config.total_supply = ctx
        .accounts
        .stablecoin_config
        .total_supply
        .checked_sub(amount)
        .ok_or(StableFunError::MathOverflow)?;

    // Emit event
    emit!(TokensRedeemed {
        mint: ctx.accounts.mint.key(),
        user: ctx.accounts.user.key(),
        amount,
        collateral_returned: collateral_amount,
    });

    Ok(())
}