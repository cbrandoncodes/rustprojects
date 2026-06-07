use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("9D9oRNLqRebSN6GFAweZ5a4HncoSTuAfuXSL2bUK6xSA");

#[program]
pub mod solana {
    use super::*;

    pub fn create_task(ctx: Context<CreateTask>, task_id: u64, content: String) -> Result<()> {
        create_task_handler(ctx, task_id, content)
    }

    pub fn update_task(ctx: Context<UpdateTask>, content: String) -> Result<()> {
        update_task_handler(ctx, content)
    }

    pub fn toggle_complete(ctx: Context<ToggleComplete>) -> Result<()> {
        toggle_complete_handler(ctx)
    }

    pub fn delete_task(ctx: Context<DeleteTask>) -> Result<()> {
        delete_task_handler(ctx)
    }
}