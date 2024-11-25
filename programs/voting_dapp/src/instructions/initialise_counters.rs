use anchor_lang::prelude::*;
use crate::constants;
use crate::state::{counter, registrations};

pub fn initialise_counters(ctx: Context<InitialiseCounters>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    let registrations = &mut ctx.accounts.registrations;

    counter.count = 0;
    registrations.count = 0;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitialiseCounters<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = constants::ANCHOR_DESCRIMINATOR_SIZE + counter::Counter::INIT_SPACE,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, counter::Counter>,

    #[account(
        init,
        payer = signer,
        space = constants::ANCHOR_DESCRIMINATOR_SIZE + registrations::Registrations::INIT_SPACE,
        seeds = [b"registrations"],
        bump
    )]
    pub registrations: Account<'info, registrations::Registrations>,

    pub system_program: Program<'info, System>
}