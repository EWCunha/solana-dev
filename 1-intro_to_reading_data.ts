import { Connection, PublicKey, clusterApiUrl, LAMPORTS_PER_SOL } from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"));
import { Keypair } from "@solana/web3.js";
// @ts-ignore
import * as fs from 'fs';

const secretKey = JSON.parse(fs.readFileSync('my_wallet.json', 'utf-8'));
const keypair = Keypair.fromSecretKey(new Uint8Array(secretKey));

const address = keypair.publicKey;
const balanceInLamports = await connection.getBalance(address);
const balanceInSol = balanceInLamports / LAMPORTS_PER_SOL;

console.log(`SOL balance ${balanceInSol}`);
