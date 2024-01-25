import { HttpLink } from "@apollo/client";
import { NextSSRInMemoryCache, NextSSRApolloClient } from "@apollo/experimental-nextjs-app-support/ssr";
import { registerApolloClient } from "@apollo/experimental-nextjs-app-support/rsc";

export const { getClient } = registerApolloClient(() => {
  return new NextSSRApolloClient({
    cache: new NextSSRInMemoryCache(),
    link: new HttpLink({
      uri: "http://localhost:3001/graphql",
      credentials: "include",
      fetch: (url, options) => {
        console.log("fetching", url, options);
        return fetch(url, {
          ...options,
          credentials: "include",
        });
      },
    }),
  });
});