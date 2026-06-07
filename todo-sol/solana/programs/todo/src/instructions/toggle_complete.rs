use anchor_lang::prelude::*;

use crate::state::Task;

pub fn toggle_complete_handler(ctx: Context<ToggleComplete>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    task.completed = !task.completed;
    Ok(())
}

#[derive(Accounts)]
pub struct ToggleComplete<'info> {
    #[account(mut, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}