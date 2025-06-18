import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { WalletLocker } from "../target/types/wallet_locker";
import { PublicKey, sendAndConfirmTransaction, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { getKeypairFromFile } from "@solana-developers/helpers"
import { expect } from "chai";


describe("wallet_locker", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.walletLocker as Program<WalletLocker>;


    it("Config is initialized!", async () => {
        const timeLock = new BN(1);
        // Add your test here.
        const tx = await program.methods.initializeConfig(timeLock).rpc();
        console.log("Your transaction signature", tx);

        const [configPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("config")],
            program.programId,
        );

        const config = await program.account.config.fetch(configPDA);
        expect(config.timeLock.toNumber()).to.equal(timeLock.toNumber());
    });

    it("Time lock is set!", async () => {
        const timeLock = new BN(2);
        const [configPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("config")],
            program.programId,
        );

        const configBefore = await program.account.config.fetch(configPDA);

        const tx = await program.methods.setTimeLock(timeLock).accounts({
            config: configPDA,
        }).rpc();
        console.log("Your transaction signature", tx);

        const configAfter = await program.account.config.fetch(configPDA);
        expect(configBefore.timeLock.toNumber()).to.not.equal(
            configAfter.timeLock.toNumber(),
        );
        expect(configAfter.timeLock.toNumber()).to.equal(timeLock.toNumber());
    });

    it("Should revert if time lock is called by non-authority", async () => {
        const timeLock = new BN(2);
        const [configPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("config")],
            program.programId,
        );

        const authority = anchor.web3.Keypair.generate();

        const timeLockTx = await program.methods.setTimeLock(timeLock).accounts({
            config: configPDA,
        }).transaction();

        try {
            await sendAndConfirmTransaction(provider.connection, timeLockTx, [authority]);
            throw new Error("Transaction should have been reverted");
        } catch (error) {
            expect(error).to.be.instanceOf(Error, "Transaction should have been reverted");
        }
    });

    it("Wallet is initialized!", async () => {
        const tx = await program.methods.initializeWallet().rpc();
        console.log("Your transaction signature", tx);

        const [walletPDA] = PublicKey.findProgramAddressSync([Buffer.from("wallet"), provider.wallet.publicKey.toBuffer()], program.programId);
        const walletAccount = await program.account.wallet.fetch(walletPDA);

        expect(walletAccount.lastDeposit.toNumber()).to.equal(0);
    })

    it("Deposit", async () => {
        const depositAmount = new BN(1000);
        const [walletPDA] = PublicKey.findProgramAddressSync([Buffer.from("wallet"), provider.wallet.publicKey.toBuffer()], program.programId);
        const [configPDA] = PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId);
        const config = await program.account.config.fetch(configPDA);

        const walletPDAInfoBefore = await provider.connection.getAccountInfo(walletPDA);
        const walletBefore = await program.account.wallet.fetch(walletPDA);

        const tx = await program.methods.deposit(depositAmount).accounts({
            config: configPDA,
        }).rpc();
        console.log("Your transaction signature", tx);

        await new Promise((resolve) => setTimeout(resolve, 1_000));

        const walletPDAInfoAfter = await provider.connection.getAccountInfo(walletPDA);
        const walletAfter = await program.account.wallet.fetch(walletPDA);

        const txInfo = await provider.connection.getTransaction(tx, {
            commitment: "confirmed",
            maxSupportedTransactionVersion: 0
        });

        expect(walletPDAInfoBefore.lamports).to.not.equal(0);
        expect(walletPDAInfoAfter.lamports).to.equal(walletPDAInfoBefore.lamports + depositAmount.toNumber());

        expect(walletBefore.lastDeposit.toNumber()).to.equal(0);
        expect(txInfo.blockTime + config.timeLock.toNumber()).to.be.greaterThanOrEqual(walletAfter.lastDeposit.toNumber());

    })

    it("Withdraw", async () => {
        const withdrawAmount = new BN(1000);
        const [walletPDA] = PublicKey.findProgramAddressSync([Buffer.from("wallet"), provider.wallet.publicKey.toBuffer()], program.programId);
        const [configPDA] = PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId);

        await new Promise((resolve) => setTimeout(resolve, 3_000));

        const walletBefore = await provider.connection.getAccountInfo(walletPDA);

        const tx = await program.methods.withdraw(withdrawAmount).accounts({
            config: configPDA,
            signer: provider.wallet.publicKey,
        }).rpc();

        const walletAfter = await provider.connection.getAccountInfo(walletPDA);

        expect(walletBefore.lamports).to.not.equal(0);
        expect(walletAfter.lamports).to.equal(walletBefore.lamports - withdrawAmount.toNumber());

    })
});
