use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Counter::size(),
        seeds = [
            PREFIX,
            COUNTER,
        ],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeCounter>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.bump = ctx.bumps.counter;
    counter.id = 1;

    Ok(())
}
