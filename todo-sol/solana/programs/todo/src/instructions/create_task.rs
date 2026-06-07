use anchor_lang::prelude::*;

use crate::{errors::TodoError, state::Task};

pub fn create_task_handler(ctx: Context<CreateTask>, task_id: u64, content: String) -> Result<()> {
    validate_content(&content)?;

    let task = &mut ctx.accounts.task;
    let author = &ctx.accounts.author;

    task.task_id = task_id;
    task.content = content;
    task.completed = false;
    task.author = author.key();
    task.created_at = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CreateTask<'info> {
    #[account(mut)]
    pub author: Signer<'info>,

    #[account(
        init,
        payer = author,
        space = Task::SPACE,
        seeds = [b"task", author.key().as_ref(), &task_id.to_le_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    pub system_program: Program<'info, System>,
}

pub fn validate_content(content: &str) -> Result<()> {
    require!(
        !content.is_empty() && content.len() <= Task::MAX_CONTENT_LEN,
        TodoError::InvalidContent
    );

    Ok(())
}