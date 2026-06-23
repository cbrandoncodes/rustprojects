use anchor_lang::prelude::*;

use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, COUNTER_SEED, REGISTRATIONS_SEED},
    state::{Counter, Registrations},
};

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.count = 0;

    let registrations = &mut ctx.accounts.registrations;
    registrations.count = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8,
        seeds = [COUNTER_SEED],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8,
        seeds = [REGISTRATIONS_SEED],
        bump
    )]
    pub registrations: Account<'info, Registrations>,

    pub system_program: Program<'info, System>,
}
