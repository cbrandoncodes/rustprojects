use anchor_lang::prelude::*;

use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, REGISTRATIONS_SEED, VOTER_SEED},
    errors::ErrorCode,
    state::{Candidate, Poll, Registrations, Voter},
};

pub fn handle_vote(ctx: Context<VoteCandidate>, poll_id: u64, cid: u64) -> Result<()> {
    let voter = &mut ctx.accounts.voter;
    let candidate = &mut ctx.accounts.candidate;
    let poll = &mut ctx.accounts.poll;

    if !candidate.has_registered || candidate.poll_id != poll_id {
        return Err(ErrorCode::CandidateNotRegistered.into());
    }

    if voter.has_voted {
        return Err(ErrorCode::VoterAlreadyVoted.into());
    }

    let current_time: u64 = Clock::get()?.unix_timestamp as u64;
    if current_time < poll.start || current_time > poll.end {
        return Err(ErrorCode::PollNotActive.into());
    }

    voter.poll_id = poll_id;
    voter.cid = cid;
    voter.has_voted = true;

    candidate.votes += 1;

    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64, cid: u64)]
pub struct VoteCandidate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [
            poll_id.to_le_bytes().as_ref(),
            cid.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub candidate: Account<'info, Candidate>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Voter::INIT_SPACE,
        seeds = [
            VOTER_SEED,
            poll_id.to_le_bytes().as_ref(),
            user.key().as_ref()
        ],
        bump
    )]
    pub voter: Account<'info, Voter>,

    #[account(
        mut,
        seeds = [REGISTRATIONS_SEED],
        bump
    )]
    pub registrations: Account<'info, Registrations>,

    pub system_program: Program<'info, System>,
}
