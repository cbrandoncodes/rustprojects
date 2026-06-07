use anchor_lang::prelude::*;

use crate::{instructions::validate_content, state::Task};

pub fn update_task_handler(ctx: Context<UpdateTask>, content: String) -> Result<()> {
    validate_content(&content)?;
    ctx.accounts.task.content = content;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}