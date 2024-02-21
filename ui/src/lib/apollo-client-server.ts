import { HttpLink } from '@apollo/client'
import {
  NextSSRInMemoryCache,
  NextSSRApolloClient,
} from '@apollo/experimental-nextjs-app-support/ssr'
import { registerApolloClient } from '@apollo/experimental-nextjs-app-support/rsc'
import { cookies } from 'next/headers'
import config from './config'

export const { getClient: getServerClient } = registerApolloClient(() => {
  return new NextSSRApolloClient({
    cache: new NextSSRInMemoryCache(),
    link: new HttpLink({
      uri: `${config.apiBaseUrl}/graphql`,
      fetch: async (url, options) => {
        const token = cookies().get('token')

        return await fetch(url, {
          ...options,
          headers: {
            ...(options?.headers ?? {}),
            Authorization: `Bearer ${token?.value}`,
          },
        })
      },
    }),
  })
})
