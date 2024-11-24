use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("The poll does not exist")]
    NonExistentPoll,
    #[msg("Poll end time must be greater than the start time")]
    InvalidEndTime
}