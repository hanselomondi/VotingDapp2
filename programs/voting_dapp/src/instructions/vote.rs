use anchor_lang::prelude::*;
use crate::state::{voter, candidate, poll};
use crate::constants;
use crate::errors::VotingDappErrors;

pub fn vote(
    ctx: Context<Vote>,
    candidate_id: u64,
    poll_id: u64
) -> Result<()> {
    let candidate = &mut ctx.accounts.candidate;
    let poll = &mut ctx.accounts.poll;  
    let voter = &mut ctx.accounts.voter;
    let current_time = Clock::get()?.unix_timestamp as u64;

    if current_time > poll.poll_end {
        return Err(VotingDappErrors::PollClosed.into())
    }
    if current_time < poll.poll_start {
        return Err(VotingDappErrors::PollNotActive.into())
    }
    if voter.has_voted {
        return Err(VotingDappErrors::AlreadyVoted.into())
    }
    
    candidate.total_votes += 1;
    voter.has_voted = true;
    voter.candidate_id = candidate_id;
    voter.poll_id = poll_id;

    Ok(())
}

#[derive(Accounts)]
#[instruction(candidate_id: u64, poll_id: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = constants::ANCHOR_DESCRIMINATOR_SIZE + voter::Vote::INIT_SPACE,
        seeds = [
            candidate_id.to_le_bytes().as_ref(),
            poll_id.to_le_bytes().as_ref(),
            signer.key().as_ref()
        ],
        bump
    )]
    pub voter: Account<'info, voter::Vote>,

    #[account(
        mut,
        seeds = [
            b"candidate",
            poll_id.to_le_bytes().as_ref(),
            candidate_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub candidate: Account<'info, candidate::Candidate>,

    #[account(
        seeds = [
            b"poll",
            poll_id.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub poll: Account<'info, poll::Poll>,

    pub system_program: Program<'info, System>
}