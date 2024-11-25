use anchor_lang::prelude::*;

use crate::errors::VotingDappErrors;
use crate::constants;
use crate::state::{candidate, poll, registrations};

pub fn register_candidate(
    ctx: Context<RegisterCandidate>,
    candidate_name: String,
    poll_id: u64
) -> Result<()> {
    let candidate = &mut ctx.accounts.candidate;
    let registrations = &mut ctx.accounts.registrations;
    let poll = &mut ctx.accounts.poll;

    if poll_id != poll.poll_id {
        return Err(VotingDappErrors::InvalidPollIdProvided.into())
    }

    registrations.count += 1;
    poll.candidate_count += 1;

    candidate.candidate_id = registrations.count;
    candidate.name = candidate_name;
    candidate.poll_id = poll.poll_id;
    candidate.total_votes = 0;
    msg!("Candidate successfully registered");

    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct RegisterCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, poll::Poll>,

    #[account(
        seeds = [b"counter"],
        bump
    )]
    pub registrations: Account<'info, registrations::Registrations>,

    #[account(
        init,
        payer = signer,
        space = constants::ANCHOR_DESCRIMINATOR_SIZE + candidate::Candidate::INIT_SPACE,
        seeds = [
            b"candidate",
            poll_id.to_le_bytes().as_ref(),
            (registrations.count + 1).to_le_bytes().as_ref()
        ],
        bump
    )]
    pub candidate: Account<'info, candidate::Candidate>,

    pub system_program: Program<'info, System>
}