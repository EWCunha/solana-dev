import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { FirstAnchorProgram } from "../target/types/first_anchor_program";
import { expect } from "chai";

describe("first-anchor-program", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.firstAnchorProgram as Program<FirstAnchorProgram>;

    const counter = anchor.web3.Keypair.generate();
    it("Is initialized!", async () => {
        // Add your test here.
        const tx = await program.methods.initialize(new BN(10)).accounts({
            counter: counter.publicKey,
        }).signers([counter]).rpc();
        console.log("Your transaction signature", tx);

        const counterAccount = await program.account.counter.fetch(counter.publicKey);
        expect(counterAccount.count.toNumber()).to.equal(10);
    });

    it("Is incremented!", async () => {
        const tx = await program.methods.increment().accounts({
            counter: counter.publicKey,
            user: provider.wallet.publicKey,
        }).rpc();
        console.log("Your transaction signature", tx);

        const counterAccount = await program.account.counter.fetch(counter.publicKey);
        expect(counterAccount.count.toNumber()).to.equal(11);
    });
});
