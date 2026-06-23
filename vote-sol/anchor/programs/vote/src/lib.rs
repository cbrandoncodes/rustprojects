use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("DmVBj2kXM6536x319mKuWYsje55PpEaBkXcr8q1rLxZ7");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::handle_initialize(ctx)
    }

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        description: String,
        start: u64,
        end: u64,
    ) -> Result<()> {
        instructions::handle_create_poll(ctx, description, start, end)
    }

    pub fn register_candidate(
        ctx: Context<RegisterCandidate>,
        poll_id: u64,
        name: String,
    ) -> Result<()> {
        instructions::handle_register_candidate(ctx, poll_id, name)
    }

    pub fn vote(ctx: Context<VoteCandidate>, poll_id: u64, cid: u64) -> Result<()> {
        instructions::handle_vote(ctx, poll_id, cid)
    }
}
