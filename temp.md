Unit 0 - Introduction
1 - Get started

Unit 1 - dApp development
1 - Introduction to cryptography and Solana clients
2 - Cryptography fundamentals
    - Lab: Create and load keypairs
3 - Read data from the network
    - Lab: Connect to Solana and check account balances
4 - Create transactions on the Solana Network
    - Lab: Make transactions and use Explorer
5 - Create tokens with the Token Program
    - Lab: Mint, transfer and burn tokens
6 - Create Solana NFTs With Metaplex
    - Lab: Mint NFTs, update their metadata, and associate with a collection
7 - Create transactions for custom onchain programs
    - Lab: Make transactions for a custom onchain program
8 - Interact with wallets
    - Lab: Make a React frontend to the Ping program

Unit 2 - Onchain program development
1 - Introduction to Onchain Program Development
2 - Local Setup
    - Lab: Configure Windows, Mac and Linux for localhost Solana development

Unit 3 - Anchor Program Development
1 - Getting started with Anchor
    - Lab: Create a 'hello world' application
2 - Using Anchor programs from JS/TS
    - Lab: Create a React and Anchor frontend for the ping-counter app
3 - Anchor PDAs and accounts
    - Lab: Use Anchor to create a Movie Review program
4 - Anchor CPIs and errors
    - Lab: Mint token rewards for reviews

Unit 4 - Native Solana Program Development
1 - Serialize program data
    - Lab: Build a React frontend to submit movie reviews
2 - Deserialize program data
    - Lab: Build a React frontend to show movie reviews
3 - Page, Order, and Filter program data
    - Lab: Adding paging, ordering and searching to the movie reviews frontend
4 - Hello World in native
    - Lab: Create and deploy a native 'hello world' onchain program
5 - Handle Instruction Data
    - Lab: Make an onchain program that recieves movie reviews and deserialises them
6 - State Management
    - Lab: Make an onchain program that stores movie reviews
7 - Basic Security and Validation
    - Lab: Add security checks and updates to the movie reviews program

Unit 5 - Intermediate Solana Program Development
1 - Program Derived Addresses
    - Lab: Use PDAs to store comments for the movie reviews program
2 - Cross Program Invocations
    - Lab: Use CPIs to mint token rewards for reviews and comments
3 - Program Testing

Unit 6 - Beyond the Basics
1 - Environment variables in Solana programs
    - Lab: Create feature flags to help test locally
2 - Solana Pay
    - Lab: Use QR codes to create payment requests
3 - Versioned transactions and lookup tables
    - Lab: Using a lookup table to handle many accounts in a single transaction
4 - Rust procedural macros
    - Lab: Use Rust macros to generate program code

Unit 7 - Solana Program Security
1 - How to approach the Program Security unit
2 - Signer authorization
    - Lab: Check signers are correct before processing transactions
3 - Owner checks
    - Lab: Check owners are correct before processing transactions
4 - Account data matching
    - Lab: Check account data is correct before processing transactions
5 - Reinitialization attacks
    - Lab: Ensure accounts can only be initialised once to prevent overriding existing data
6 - Duplicate mutable accounts
    - Lab: Ensure accounts are distinct
7 - Type cosplay
    - Lab: Implement checks for correct account types
8 - Arbitrary CPIs
    - Lab: Ensure CPIs are only called by the correct program
9 - Bump seed canonicalization
    - Lab: Ensure bump seeds are always canonical
10 - Closing accounts and revival attacks
    - Lab: Ensure accounts that are closed can't be reused
11 - PDA sharing
    - Lab: Ensure PDA design avoids shared accounts

Unit 8 - Advanced Solana Programming
1 - Program architecture
    - Lab: Create an optimized onchain RPG game
2 - Oracles and oracle networks
    - Lab: Use Switchboard to create an Escrow program
3 - Verifiable randomness functions
    - Lab: Add randomness to the Escrow program
4 - Compressed NFTs
    - Lab: Mint, read and transfer NFTs at scale with Bubblegum
5 - Generalized State Compression
    - Lab: Build a Notes program using state compression
6 - Durable Nonces
    - Lab: Use durable nonces to create durable transactions

Unit 9 - Solana Mobile Development
1 - Introduction to Solana Mobile
    - Lab: Build a simple Android counter dApp with React Native
2 - Exploring Mobile Wallet Adapter
    - Lab: Build a simple MWA-enabled mobile wallet
3 - Building Solana Mobile dApps with Expo
    - Lab: Build a mobile dApp that mints NFTs from photos

Unit 10 - Token Extensions Program
1 - Intro to Token Extensions Program
    - Lab: Use the CLI to interact with the Token Extensions Program
2 - Intro to Token Extensions Program in the Client
    - Lab: Integrate Token Program and Token Extensions Program in the client
3 - Intro to Token Extensions Program On-Chain
    - Lab: Integrate Token Program and Token Extensions Program on-chain

Unit 11 - Mint Extensions
1 - Metadata and Metadata Pointer Extension
    - Lab: Make an NFT using the Token Extensions Program
2 - Group, Group Pointer, Member, and Member Pointer Extensions
    - Lab: Create an NFT collection with group, member and metadata extensions
3 - Non-Transferrable Token Extension
    - Lab: Create a soul-bound NFT
4 - Transfer Fee Extension
    - Lab: Create a mint with transfer fees enabled
5 - Close Mint Extension
    - Lab: Create a closable mint
6 - Interest Bearing Token Extension
    - Lab: Create an interest bearing token
7 - Permanent Delegate Extension
    - Lab: Create a mint with a permanent delegate
8 - Transfer Hook Extension
    - Lab: Create a mint with a transfer hook

Unit 12 - Token Extensions
1 - Default State Extension
    - Lab: Test token interactions with frozen default states
2 - Immutable Owner Extension
    - Lab: Create a mint with an immutable owner
3 - Required Memo Extension
    - Lab: Create a token that requires a memo
4 - CPI Guard Extension
    - Lab: Create a token that requires a CPI guard