use crate::state::*;
use anchor_lang::prelude::*;

pub fn create_proposal_handler(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    proposal.title = title;
    proposal.description = description;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.creator = *ctx.accounts.user.key;
    Ok(())
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(
        init,
        seeds = [title.as_bytes()],
        bump,
        payer = user,
        space = 8 + Proposal::INIT_SPACE
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
