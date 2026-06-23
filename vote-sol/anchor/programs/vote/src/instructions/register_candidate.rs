use anchor_lang::prelude::*;

use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, REGISTRATIONS_SEED},
    errors::ErrorCode,
    state::{Candidate, Poll, Registrations},
};

pub fn handle_register_candidate(
    ctx: Context<RegisterCandidate>,
    poll_id: u64,
    name: String,
) -> Result<()> {
    let poll = &mut ctx.accounts.poll;
    if poll.id != poll_id {
        return Err(ErrorCode::PollNotFound.into());
    }

    let candidate = &mut ctx.accounts.candidate;
    if candidate.has_registered {
        return Err(ErrorCode::CandidateRegistered.into());
    }

    let registrations = &mut ctx.accounts.registrations;
    registrations.count += 1;

    candidate.cid = registrations.count;
    candidate.poll_id = poll_id;
    candidate.name = name;
    candidate.has_registered = true;

    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct RegisterCandidate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Candidate::INIT_SPACE,
        seeds = [
            poll_id.to_le_bytes().as_ref(),
            (registrations.count + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        mut,
        seeds = [REGISTRATIONS_SEED],
        bump
    )]
    pub registrations: Account<'info, Registrations>,

    pub system_program: Program<'info, System>,
}
