use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    #[max_len(20)]
    pub title: String,
    #[max_len(200)]
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub creator: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct ProposalCounter {
    pub current: u64,
}

#[account]
#[derive(InitSpace)]
pub struct UserVote {
    #[max_len(20)]
    pub proposal_title: String,
    pub voter: Pubkey,
    pub voted: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Voter {
    pub pubkey: Pubkey,
    pub reward_points: u64,
}
