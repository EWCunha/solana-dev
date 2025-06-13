import { Keypair, PublicKey, clusterApiUrl, Connection, Transaction, SystemProgram, LAMPORTS_PER_SOL, sendAndConfirmTransaction, TransactionInstruction } from "@solana/web3.js";
import { getKeypairFromFile, airdropIfRequired } from "@solana-developers/helpers";

const amount = 0.1;
const airdropAmount = 5;
const senderKeypair = await getKeypairFromFile("./my_wallet.json");
const receiverPublicKey = new PublicKey('C5vrZkxKz5enLxEYd2V5TWqV8SthJZSGtsNSBmcpqg9K');
const connection = new Connection(clusterApiUrl("devnet"));

const transaction = new Transaction()

const sendSolInstruction = SystemProgram.transfer({
    fromPubkey: senderKeypair.publicKey,
    toPubkey: receiverPublicKey,
    lamports: LAMPORTS_PER_SOL * amount
})

transaction.add(sendSolInstruction)

// const airdropSignature = await connection.requestAirdrop(senderKeypair.publicKey, LAMPORTS_PER_SOL * airdropAmount)
// console.log(`Airdrop signature: ${airdropSignature}`);
// await airdropIfRequired(
//     connection,
//     senderKeypair.publicKey,
//     LAMPORTS_PER_SOL * airdropAmount,
//     LAMPORTS_PER_SOL * amount,
// );

const instructionData = Buffer.alloc(4 + 8);
instructionData.writeUInt32LE(2, 0)
instructionData.writeBigUInt64LE(BigInt(LAMPORTS_PER_SOL * amount), 4)

const manualInstruction = new TransactionInstruction({
    keys: [
        {
            pubkey: senderKeypair.publicKey,
            isSigner: true,
            isWritable: true
        },
        {
            pubkey: receiverPublicKey,
            isSigner: false,
            isWritable: true
        }
    ],
    programId: SystemProgram.programId,
    data: instructionData
});
transaction.add(manualInstruction)


const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [senderKeypair]
)

console.log(`Transaction sent: ${signature}`);