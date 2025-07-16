use anchor_lang::prelude::*;

declare_id!("7ZhkpufuaTjbdVTmrtwyZEkhtGpnghcQ55QHfSqmgC8j");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
