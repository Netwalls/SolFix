use anchor_lang::prelude::*;

declare_id!("4g4JPJgeD3V8sgbzKrjsa5zW2XiVvsNdKTzY2UhdhMaN");

#[program]
pub mod stable_fun {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
