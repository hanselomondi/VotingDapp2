use anchor_lang::prelude::*;

#[error_code]
pub enum VotingDappErrors {
    #[msg("The poll does not exist or has not been initialised")]
    NonExistentPoll,
    #[msg("Poll end time must be greater than the start time")]
    InvalidEndTime,
    #[msg("Poll cannot end before current time")]
    InvalidPollTime,
    #[msg("Invalid poll id provided")]
    InvalidPollIdProvided,
    #[msg("Poll not active.")]
    PollNotActive,
    #[msg("Poll has closed")]
    PollClosed,
    #[msg("You cannot vote more than once")]
    AlreadyVoted
}