use anchor_lang::prelude::*;

#[error_code]
pub enum TodoError {
    #[msg("Task content must be between 1 and 200 bytes.")]
    InvalidContent,
}