use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Start date cannot be greater than end date")]
    InvalidDates,

    #[msg("Poll was not found")]
    PollNotFound,

    #[msg("Candidate already registered")]
    CandidateRegistered,

    #[msg("Candidate not registered")]
    CandidateNotRegistered,

    #[msg("Voter has already voted")]
    VoterAlreadyVoted,

    #[msg("Poll not active")]
    PollNotActive,
}
