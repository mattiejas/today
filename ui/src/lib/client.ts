import { HttpLink } from "@apollo/client";
import { NextSSRInMemoryCache, NextSSRApolloClient } from "@apollo/experimental-nextjs-app-support/ssr";
import { registerApolloClient } from "@apollo/experimental-nextjs-app-support/rsc";
import { cookies } from "next/headers";
import config from "./config";

export const { getClient } = registerApolloClient(() => {
  return new NextSSRApolloClient({
    cache: new NextSSRInMemoryCache(),
    link: new HttpLink({
      uri: `${config.apiBaseUrl}/graphql`,
      fetch: (url, options) => {
        const token = cookies().get("token");

        return fetch(url, {
          ...options,
          headers: {
            ...(options?.headers ?? {}),
            Authorization: `Bearer ${token?.value}`,
          },
        });
      },
    }),
  });
});
