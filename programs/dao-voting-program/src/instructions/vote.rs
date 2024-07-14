use crate::state::*;
use anchor_lang::prelude::*;

pub fn vote_handler(ctx: Context<Vote>, title: String, vote: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    let user_vote = &mut ctx.accounts.user_vote;

    if vote {
        proposal.yes_votes += 1;
    } else {
        proposal.no_votes += 1;
    }

    user_vote.voted = true;
    user_vote.voter = ctx.accounts.user.key();
    user_vote.proposal_title = title;

    Ok(())
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes()],
        bump,
        realloc = 8 + Proposal::INIT_SPACE,
        realloc::payer = user,
        realloc::zero = true
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init,
        seeds = [title.as_bytes(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + UserVote::INIT_SPACE
    )]
    pub user_vote: Account<'info, UserVote>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
