use anchor_lang::prelude::*;

#[account]
pub struct Task {
    pub task_id: u64,
    pub content: String,
    pub completed: bool,
    pub author: Pubkey,
    pub created_at: i64,
}

impl Task {
    pub const MAX_CONTENT_LEN: usize = 200;
    pub const SPACE: usize = 8 + 8 + 4 + Self::MAX_CONTENT_LEN + 1 + 32 + 8;
}