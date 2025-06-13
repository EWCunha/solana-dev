import { createAccount, createMint, mintTo, getAssociatedTokenAddress, getAssociatedTokenAddressSync, transfer, burn } from "@solana/spl-token"
import { Connection, clusterApiUrl, Transaction, TransactionSignature } from "@solana/web3.js"
import { getKeypairFromFile, airdropIfRequired } from "@solana-developers/helpers";

const connection = new Connection(clusterApiUrl("devnet"))
const senderKeypair = await getKeypairFromFile("./my_wallet.json")
const tokenMintKeypair = await getKeypairFromFile("./shomer.json")
const tokenAccountKeypair = await getKeypairFromFile("./shomer_ta.json")

const decimals = 6

const mint = await createMint(connection, senderKeypair, senderKeypair.publicKey, senderKeypair.publicKey, decimals, tokenMintKeypair)
console.log(`Mint: ${mint.toBase58()}`)

const tokenAccount = await createAccount(connection, senderKeypair, mint, senderKeypair.publicKey, tokenAccountKeypair)
console.log(`Token Account: ${tokenAccount.toBase58()}`)

const associatedTokenAccount = await createAccount(connection, senderKeypair, mint, senderKeypair.publicKey)
console.log(`Associated Token Account: ${associatedTokenAccount.toBase58()}`)

const associatedTokenAccountGot = await getAssociatedTokenAddress(mint, senderKeypair.publicKey)
console.log(`Associated Token Account Got: ${associatedTokenAccountGot.toBase58()}`)

const calculatedAssociatedTokenAccount = getAssociatedTokenAddressSync(mint, senderKeypair.publicKey)
console.log(`Associated Calculated Token Account: ${calculatedAssociatedTokenAccount.toBase58()}`)

const amountToMint = 1_000_000 * 10 ** decimals  // 1M
const mintTx = await mintTo(connection, senderKeypair, mint, associatedTokenAccount, senderKeypair, amountToMint)
console.log(`Mint Tx: ${mintTx}`)

const amountToTransfer = 100 * 10 ** decimals  // 100   
const transferTx = await transfer(connection, senderKeypair, associatedTokenAccount, tokenAccount, senderKeypair, amountToTransfer)
console.log(`Transfer Tx: ${transferTx}`)

const amoutToBurn = 10 * 10 ** decimals  // 10
const burnTx = await burn(connection, senderKeypair, associatedTokenAccount, mint, senderKeypair, amoutToBurn)
console.log(`Burn Tx: ${burnTx}`)

