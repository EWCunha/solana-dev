import { PublicKey } from "@solana/web3.js";

const programId = new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")
const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("GLOBAL_STATE")], programId)

console.log(`PDA: ${pda.toBase58()}, Bump: ${bump}`)

const pda2 = PublicKey.createProgramAddressSync([Buffer.from("GLOBAL_STATE"), Buffer.from([bump])], programId)

console.log(`PDA2: ${pda2.toBase58()}`)
console.log(`is on curve: ${PublicKey.isOnCurve(pda2)}`)

const userPublicKey = new PublicKey("FobH7JPsxFxwY283eBz4UaaFa3z6qm4NydnMNdy7B8JX")
const [pda3, bump3] = PublicKey.findProgramAddressSync([userPublicKey.toBuffer(), Buffer.from("USER_DATA"),], programId)

console.log(`PDA3: ${pda3.toBase58()}, Bump3: ${bump3}`)
