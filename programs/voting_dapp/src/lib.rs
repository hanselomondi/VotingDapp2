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
    use super::*;

    pub fn initialise(ctx: Context<InitialiseCounters>) -> Result<()> {
        initialise_counters::initialise_counters(ctx)
    }
}