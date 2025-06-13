import * as web3 from '@solana/web3.js'
import { TokenSwap, TOKEN_SWAP_PROGRAM_ID, TokenSwapLayout, CurveType } from "@solana/spl-token-swap"
import { getKeypairFromFile } from '@solana-developers/helpers'
import * as token from "@solana/spl-token"
import { sendAndConfirmTransaction } from '@solana/web3.js'

const connection = new web3.Connection(web3.clusterApiUrl("devnet"))
const wallet = await getKeypairFromFile("./my_wallet.json")

const transaction = new web3.Transaction()
const tokenSwapStateAccount = await getKeypairFromFile("./tokenSwapStateAccount.json")
const rent = await TokenSwap.getMinBalanceRentForExemptTokenSwap(connection)
const tokenSwapStateAccountInstruction = web3.SystemProgram.createAccount({
    newAccountPubkey: tokenSwapStateAccount.publicKey,
    fromPubkey: wallet.publicKey,
    lamports: rent,
    space: TokenSwapLayout.span,
    programId: TOKEN_SWAP_PROGRAM_ID
})
transaction.add(tokenSwapStateAccountInstruction)

const [swapAuthority, bump] = web3.PublicKey.findProgramAddressSync(
    [tokenSwapStateAccount.publicKey.toBuffer()],
    TOKEN_SWAP_PROGRAM_ID,
)

const tokenAMint = new web3.PublicKey("DQisQUvsCRsFr5ewYG8ojWRYuJSXVEXDCMAMW2xmLfWz")
const tokenBMint = new web3.PublicKey("Hc1uAJGG3t4PuVt2kGjbQzMbbJ39f3r4tnQK7e6JNJ4q")

let tokenAAccountAddress = await token.getAssociatedTokenAddress(
    tokenAMint,
    swapAuthority,
    true
)

const tokenAAccountInstruction = token.createAssociatedTokenAccountInstruction(
    wallet.publicKey,
    tokenAAccountAddress,
    swapAuthority,
    tokenAMint
)

transaction.add(tokenAAccountInstruction)

let tokenBAccountAddress = await token.getAssociatedTokenAddress(
    tokenBMint,
    swapAuthority,
    true
)

const tokenBAccountInstruction = token.createAssociatedTokenAccountInstruction(
    wallet.publicKey,
    tokenBAccountAddress,
    swapAuthority,
    tokenBMint
)

transaction.add(tokenBAccountInstruction)

const poolTokenMint = new web3.PublicKey("HgnYWtB3WGDuNn7oRrAoc6NSazWhnsGMT9bFDTaVJqHL")
const tokenAccountPool = await getKeypairFromFile("./token_account_pool.json")
const poolAccountRent = await token.getMinimumBalanceForRentExemptAccount(connection)
const createTokenAccountPoolInstruction = web3.SystemProgram.createAccount({
    fromPubkey: wallet.publicKey,
    newAccountPubkey: tokenAccountPool.publicKey,
    space: token.ACCOUNT_SIZE,
    lamports: poolAccountRent,
    programId: token.TOKEN_PROGRAM_ID,
})
const initializeTokenAccountPoolInstruction = token.createInitializeAccountInstruction(
    tokenAccountPool.publicKey,
    poolTokenMint,
    wallet.publicKey
)

transaction.add(createTokenAccountPoolInstruction)
transaction.add(initializeTokenAccountPoolInstruction)

const feeOwner = new web3.PublicKey('HfoTxFR1Tm6kGmWgYWD6J7YHVy1UwqSULUGVLXkJqaKN')

let tokenFeeAccountAddress = await token.getAssociatedTokenAddress(
    poolTokenMint,
    feeOwner,
    true
)

const tokenFeeAccountInstruction = token.createAssociatedTokenAccountInstruction(
    wallet.publicKey,
    tokenFeeAccountAddress,
    feeOwner,
    poolTokenMint
)

transaction.add(tokenFeeAccountInstruction)

const createSwapInstruction = TokenSwap.createInitSwapInstruction(
    tokenSwapStateAccount,      // Conta de estado da troca de tokens
    swapAuthority,              // Autoridade do pool de troca
    tokenAAccountAddress,                 // Conta do token A
    tokenBAccountAddress,                 // Conta do token B
    poolTokenMint,              // Cunhagem do token do pool de troca
    tokenFeeAccountAddress,     // Conta de taxa do token
    tokenAccountPool.publicKey, // Conta do token do pool de troca
    token.TOKEN_PROGRAM_ID,     // ID do Programa de Tokens
    TOKEN_SWAP_PROGRAM_ID,      // ID do Programa de Troca de Tokens
    BigInt(0),                          // Numerador da taxa de troca
    BigInt(10000),                      // Denominador da taxa de troca
    BigInt(5),                          // Numerador da taxa de troca do proprietário
    BigInt(10000),                      // Denominador da taxa de troca do proprietário
    BigInt(0),                          // Numerador da taxa de retirada do proprietário
    BigInt(0),                          // Denominador da taxa de retirada do proprietário
    BigInt(20),                         // Numerador da taxa de hospedagem
    BigInt(100),                        // Denominador da taxa de hospedagem
    CurveType.ConstantProduct   // Tipo de curva
)
transaction.add(createSwapInstruction)

const signature = await sendAndConfirmTransaction(connection, transaction, [wallet, tokenSwapStateAccount, tokenAccountPool])
console.log(`Signature: ${signature}`)



