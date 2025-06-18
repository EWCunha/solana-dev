import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { PingCounter } from "../target/types/ping_counter";
import { expect } from "chai";

describe("ping_counter", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.pingCounter as Program<PingCounter>;

    const account = anchor.web3.Keypair.generate()

    it("Is initialized!", async () => {
        const initialCount = 10;

        // Add your test here.
        const tx = await program.methods.initialize(new BN(initialCount)).accounts({
            counter: account.publicKey,
        }).signers([account]).rpc();
        console.log("Your transaction signature", tx);
    });

    it("Is incremented!", async () => {
        const counterBefore = await program.account.counter.fetch(account.publicKey);

        const tx = await program.methods.increment().accounts({
            counter: account.publicKey,
        }).rpc()

        const counterAfter = await program.account.counter.fetch(account.publicKey);

        expect(counterAfter.count.toNumber()).to.equal(counterBefore.count.toNumber() + 1);
    });
})
