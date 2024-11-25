use anchor_lang::prelude::*;
use crate::state::{poll, counter};
use crate::constants;
use crate::errors::VotingDappErrors;

pub fn initialise_poll(
    ctx: Context<InitialisePoll>,
    poll_description: String,
    poll_start: u64,
    poll_end: u64
) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    let poll = &mut ctx.accounts.poll;
    let current_time = Clock::get()?.unix_timestamp as u64;

    if poll_start < current_time && poll_end < current_time {
        return Err(VotingDappErrors::InvalidPollTime.into());
    }
    if poll_end < poll_start {
        return Err(VotingDappErrors::InvalidEndTime.into())
    }
    counter.count += 1;

    poll.poll_id = counter.count;
    poll.poll_description = poll_description;
    poll.poll_start = poll_start;
    poll.poll_end = poll_end;
    poll.candidate_count = 0;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitialisePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = constants::ANCHOR_DESCRIMINATOR_SIZE + poll::Poll::INIT_SPACE,
        seeds = [b"poll", (counter.count).to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, poll::Poll>,

    #[account(
        mut,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, counter::Counter>,

    pub system_program: Program<'info, System>
}