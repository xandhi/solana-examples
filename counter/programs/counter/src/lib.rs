use anchor_lang::prelude::*;

declare_id!("7ZhkpufuaTjbdVTmrtwyZEkhtGpnghcQ55QHfSqmgC8j");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter_account.data = 0;
        msg!("Counter set to: 0");
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter_account.data += 1;
        msg!("Counter incremented to: {}.", ctx.accounts.counter_account.data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
    pub signer: Signer<'info>,
}

#[account]
pub struct CounterAccount {
    pub data: u64,
}