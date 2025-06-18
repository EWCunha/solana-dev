// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WalletLocker } from "../target/types/wallet_locker";
import { PublicKey } from "@solana/web3.js";

async function main() {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.WalletLocker as Program<WalletLocker>;

    console.log("Program deployed at:", program.programId.toString());

    // Initialize config
    const timeLock = new anchor.BN(1); // 1 second time lock
    const [configPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("config")],
        program.programId
    );

    try {
        const tx = await program.methods
            .initializeConfig(timeLock)
            .accounts({
                authority: provider.wallet.publicKey,
            })
            .rpc();

        console.log("Config initialized! Transaction signature:", tx);
    } catch (error) {
        console.error("Error initializing config:", error);
    }
};

main()
