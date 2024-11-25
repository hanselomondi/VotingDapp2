#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

mod constants;
mod state;
mod errors;
mod instructions;

use instructions::*;

declare_id!("E8a4H9J2LU5PshFfKEifsLUNREwhhrRnkLQtv4RL26vM");

#[program]
pub mod voting_dapp {
    use register_candidate::RegisterCandidate;

    use super::*;

    pub fn initialise_counters(ctx: Context<InitialiseCounters>) -> Result<()> {
        initialise_counters::initialise_counters(ctx)
    }

    pub fn initialise_poll(
        ctx: Context<InitialisePoll>,
        poll_description: String,
        poll_start: u64,
        poll_end: u64
    ) -> Result<()> {
        initialise_poll::initialise_poll(ctx, poll_description, poll_start, poll_end)
    }

    pub fn register_candidate(
        ctx: Context<RegisterCandidate>,
        candidate_name: String,
        poll_id: u64
    ) -> Result<()> {
        register_candidate::register_candidate(ctx, candidate_name, poll_id)
    }

    pub fn vote(
        ctx: Context<Vote>,
        candidate_id: u64,
        poll_id: u64
    ) -> Result<()> {
        vote::vote(ctx, candidate_id, poll_id)
    }
}