import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js"
import bs58 from "bs58"


const connection = new Connection(clusterApiUrl("devnet"))
const programId = new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")


const accountsWithoutData = await connection.getProgramAccounts(
    programId,
    {
        dataSlice: { offset: 0, length: 0 },
        filters: [
            {
                memcmp:
                {
                    offset: 5,
                    bytes: bs58.encode(Buffer.from("Andy"))
                }
            }
        ]
    }
)

const accountKeys = accountsWithoutData.map(account => account.pubkey)
console.log(accountKeys)

const paginatedKeys = accountKeys.slice(0, 10)
const accountInfos = await connection.getMultipleAccountsInfo(paginatedKeys)
const deserializedObjects = accountInfos.map((accountInfo) => {
    // put logic to deserialize accountInfo.data here
    console.log(accountInfo)
})