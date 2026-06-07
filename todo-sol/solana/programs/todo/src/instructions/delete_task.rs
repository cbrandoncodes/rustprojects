use anchor_lang::prelude::*;

use crate::state::Task;

pub fn delete_task_handler(_ctx: Context<DeleteTask>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(mut, close = author, has_one = author)]
    pub task: Account<'info, Task>,
    pub author: Signer<'info>,
}