use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vote {
    pub candidate_id: u64,
    pub poll_id: u64,
    pub has_voted: bool
}