use anchor_lang::prelude::*;

#[error_code]
pub enum VotingDappErrors {
    #[msg("The poll does not exist")]
    NonExistentPoll,
    #[msg("Poll end time must be greater than the start time")]
    InvalidEndTime,
    #[msg("Poll cannot end before current time")]
    InvalidPollTime
}