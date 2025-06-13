import * as web3 from '@solana/web3.js'
import { Movie } from '../models/Movie'
import bs58 from 'bs58'

const MOVIE_REVIEW_PROGRAM_ID = new web3.PublicKey('CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN')

export class MovieCoordinator {
    static accounts: web3.PublicKey[] = []

    static async prefetchAccounts(connection: web3.Connection, search: string) {
        const config: web3.GetProgramAccountsConfig = {
            dataSlice: { offset: 6, length: 5 },
            filters: [
                {
                    memcmp: {
                        offset: 5,
                        bytes: bs58.encode(new Uint8Array(Buffer.from(search)))
                    }
                }
            ]
        }
        const accountInfos = await connection.getProgramAccounts(MOVIE_REVIEW_PROGRAM_ID, config)
        accountInfos.toSorted((a, b) => {
            const dataA = new Uint8Array(a.account.data)
            const dataB = new Uint8Array(b.account.data)
            return Buffer.compare(dataA, dataB)
        })
        this.accounts = accountInfos.map(({ pubkey }) => pubkey)
    }

    static async fetchPage(connection: web3.Connection, page: number, perPage: number): Promise<Movie[]> {

        const accountsToFetch = this.accounts.slice((page - 1) * perPage, page * perPage)
        const accountInfos = await connection.getMultipleAccountsInfo(accountsToFetch)
        const movies = accountInfos.map((accountInfo) => {
            return Movie.deserialize(accountInfo?.data)
        }).filter((movie): movie is Movie => { return movie !== null })
        return movies
    }
}