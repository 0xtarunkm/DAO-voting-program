# DAO Voting Program

## Overview

This DAO Voting Program allows users to participate in decentralized voting on proposals. The program is built using Anchor and includes the following features:

- **Voting on Proposals**: Users can cast their votes on various proposals.
- **Display Results**: The program displays the results of the voting.
- **Reward Points**: Users are rewarded with points for their participation in the voting process.

## Features

### Voting System

- Users can create and submit proposals.
- Eligible users can vote on existing proposals.
- Each proposal's status (e.g., passed, rejected) is updated based on the votes received.

### Results Display

- Voting results are made available to all users.
- Results include the total number of votes, breakdown by option, and the final outcome.

### Reward Points

- Users earn reward points for participating in votes.
- The reward points system encourages higher engagement and participation.

## Getting Started

### Prerequisites

- Rust
- Anchor framework
- Solana CLI

### Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/tarunclub/DAO-voting-program.git
   ```

2. **Build the repository**
   ```
   anchor build
   ```
3. **Run tests**
   ```
   anchor test
   ```
