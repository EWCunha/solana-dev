import { Card } from './Card'
import { FC, useEffect, useState } from 'react'
import { Movie } from '../models/Movie'
import { Connection, clusterApiUrl, PublicKey } from '@solana/web3.js'
import { MovieCoordinator } from '../coordinators/MovieCoordinator'
import { useConnection } from '@solana/wallet-adapter-react'
import { Button, Center, HStack, Input, Spacer, Box, Flex } from '@chakra-ui/react'

const MOVIE_REVIEW_PROGRAM_ID = 'CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN'

export const MovieList: FC = () => {
    const { connection } = useConnection()
    const [movies, setMovies] = useState<Movie[]>([])
    const [page, setPage] = useState(1)
    const [search, setSearch] = useState('')

    useEffect(() => {
        MovieCoordinator.prefetchAccounts(connection, search)
        MovieCoordinator.fetchPage(
            connection,
            page,
            10
        ).then(setMovies)
    }, [page, search])

    useEffect(() => {
        (new Connection(clusterApiUrl("devnet")))
            .getProgramAccounts(new PublicKey(MOVIE_REVIEW_PROGRAM_ID))
            .then(
                accounts => {
                    const movies = accounts.map(({ account, pubkey }) => {
                        return Movie.deserialize(account.data)
                    }).filter((movie): movie is Movie => { return movie !== null })
                    setMovies(movies)
                }
            )
    }, [])

    return (
        <div>
            {
                movies.map((movie, i) => <Card key={i} movie={movie} />)
            }
            <Center>
                <Flex justify="space-between" width="100%" mt={2} mb={8} mx={4}>
                    {page > 1 ? (
                        <Button onClick={() => setPage(page - 1)}>Previous</Button>
                    ) : (
                        <div />
                    )}
                    {MovieCoordinator.accounts.length > page * 10 && (
                        <Button onClick={() => setPage(page + 1)}>Next</Button>
                    )}
                </Flex>
            </Center>
        </div>
    )
}