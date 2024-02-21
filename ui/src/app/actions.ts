'use server'

import { gql } from '@/__generated__'
import { getServerClient } from '@/lib/apollo-client-server'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'
import { type FetchResult } from '@apollo/client'
import { type GetHistoryQuery, type UpsertItemMutation } from '@gql/graphql'

export async function logout(): Promise<void> {
  cookies().delete('token')
  redirect('/login')
}

const UPSERT_TODAY_BLOCK = gql(/* GraphQL */ `
  mutation UpsertItem(
    $todayId: Uuid!
    $content: JSON!
    $todayItemId: Uuid
    $insertAt: Int
  ) {
    upsertItem(
      todayId: $todayId
      content: $content
      todayItemId: $todayItemId
      insertAt: $insertAt
    ) {
      id
      todayId
      sortOrder
      content {
        type
        payload
      }
    }
  }
`)

export async function upsertTodayBlock(
  todayId: string,
  content: {
    type: string
    payload: any
  },
  todayItemId?: string,
  insertAt?: number,
): Promise<FetchResult<UpsertItemMutation>> {
  const client = getServerClient()

  return await client.mutate({
    mutation: UPSERT_TODAY_BLOCK,
    variables: {
      todayId,
      content,
      todayItemId,
      insertAt,
    },
  })
}

const TODAY_HISTORY = gql(`
  query GetHistory($pagination: Pagination!) {
    history(pagination: $pagination) {
      id
      title
      date
      createdAt
      updatedAt
      items {
        id
        todayId
        sortOrder
        content {
          type
          payload
        }
      }
    }
  }
`)

export async function useTodayHistory(): Promise<FetchResult<GetHistoryQuery>> {
  const client = getServerClient()

  return await client.query({
    query: TODAY_HISTORY,
    variables: {
      pagination: {
        page: 0,
        limit: 10,
      },
    },
  })
}

const CREATE_TODAY = gql(`
mutation CreateToday {
  createToday {
    id
  }
}`)

export async function createToday(): Promise<void> {
  const client = getServerClient()
  await client.mutate({
    mutation: CREATE_TODAY,
  })

  redirect('/today')
}
