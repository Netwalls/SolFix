use anchor_lang::prelude::*;
use instructions::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stable_fun {
    use super::*;

    pub fn initialize_stablecoin(
        ctx: Context<InitializeStablecoin>,
        params: InitializeStablecoinParams,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, params)
    }

    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::mint::handler(ctx, amount)
    }

    pub fn redeem_tokens(
        ctx: Context<RedeemTokens>,
        amount: u64,
    ) -> Result<()> {
        instructions::redeem::handler(ctx, amount)
    }
}
