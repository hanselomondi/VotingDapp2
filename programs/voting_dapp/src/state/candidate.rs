use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    pub candidate_id: u64,
    pub poll_id: u64,
    #[max_len(50)]
    pub name: String
}