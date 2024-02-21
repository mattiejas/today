import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  type NormalizedCacheObject,
} from '@apollo/client'
import config from '@/lib/config'
import { setContext } from '@apollo/client/link/context'

const httpLink = createHttpLink({
  uri: `${config.apiBaseUrl}/graphql`,
})

const authLink = setContext((_, { headers }) => {
  // get the authentication token from local storage if it exists
  const cookies = document.cookie.split(';')
  const token = cookies
    .find((cookie) => cookie.includes('token'))
    ?.split('=')[1]

  console.log('token', token)

  // return the headers to the context so httpLink can read them
  return {
    headers: {
      ...headers,
      authorization: token !== undefined ? `Bearer ${token}` : '',
    },
  }
})

export const createApolloClient = (): ApolloClient<NormalizedCacheObject> =>
  new ApolloClient({
    link: authLink.concat(httpLink),
    cache: new InMemoryCache(),
  })
