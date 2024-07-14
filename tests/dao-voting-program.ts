import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DaoVotingProgram } from '../target/types/dao_voting_program';

describe('dao-voting-program', () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace
    .DaoVotingProgram as Program<DaoVotingProgram>;
  const wallet = provider.wallet;

  let proposal = {
    title: 'test',
    description: 'test desc',
    creator: wallet.publicKey,
    yesVotes: 0,
    noVotes: 0,
  };

  const [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(proposal.title)],
    program.programId
  );

  const [userVotePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(proposal.title), wallet.publicKey.toBuffer()],
    program.programId
  );

  const [rewardVoterPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(anchor.utils.bytes.utf8.encode('voter_account')),
      wallet.publicKey.toBuffer(),
    ],
    program.programId
  );

  it('Creates Proposal!', async () => {
    let tx = await program.methods
      .createProposal(proposal.title, proposal.description)
      .rpc();

    const account = await program.account.proposal.fetch(proposalPda);

    console.log(`
      title: ${account.title}
      description: ${account.description}
      creator: ${account.creator.toString()}
      yesVotes: ${account.yesVotes.toString()}
      noVotes: ${account.noVotes.toString()}
      `);
  });

  it('gives vote', async () => {
    let tx = await program.methods.vote(proposal.title, true).rpc();

    const account = await program.account.proposal.fetch(proposalPda);
    const account2 = await program.account.userVote.fetch(userVotePda);

    console.log(`
      title: ${account.title}
      description: ${account.description}
      creator: ${account.creator.toString()}
      yesVotes: ${account.yesVotes.toString()}
      noVotes: ${account.noVotes.toString()}
      `);

    console.log(`
      proposal_title: ${account2.proposalTitle.toString()},
      voter: ${account2.voter.toString()},
      voted: ${account2.voted}
      `);
  });

  it('rewards voter', async () => {
    let tx = await program.methods.rewardParticipation(wallet.publicKey).rpc();

    const account2 = await program.account.voter.fetch(rewardVoterPda);

    console.log(`
      voter: ${account2.pubkey.toString()},
      reward: ${account2.rewardPoints.toString()}
      `);
  });
});
