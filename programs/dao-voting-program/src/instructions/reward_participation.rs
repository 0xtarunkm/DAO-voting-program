use crate::state::*;
use anchor_lang::prelude::*;

pub fn reward_participation_handler(
    ctx: Context<RewardParticipation>,
    voter_pubkey: Pubkey,
) -> Result<()> {
    let voter = &mut ctx.accounts.voter;
    voter.reward_points += 1;
    voter.pubkey = voter_pubkey;
    Ok(())
}

#[derive(Accounts)]
#[instruction(voter_pubkey: Pubkey)]
pub struct RewardParticipation<'info> {
    #[account(
        init,
        seeds = [b"voter_account".as_ref(), voter_pubkey.key().as_ref()],
        bump,
        payer = user,
        space = 8 + Voter::INIT_SPACE,
    )]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
