"use server";

import { gql } from "@/__generated__";
import { getClient } from "@/lib/client";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export async function logout() {
  cookies().delete("token");
  redirect("/login");
}

const UPSERT_TODAY_BLOCK = gql(/* GraphQL */ `
  mutation UpsertItem($todayId: Uuid!, $content: JSON!, $todayItemId: Uuid) {
    upsertItem(todayId: $todayId, content: $content, todayItemId: $todayItemId) {
      id
      todayId
      content {
        type
        payload
      }
    }
  }
`);

export async function upsertTodayBlock(todayId: string, content: { type: string; payload: any }, todayItemId?: string) {
  const client = getClient();

  const result = await client.mutate({
    mutation: UPSERT_TODAY_BLOCK,
    variables: {
      todayId,
      content,
      todayItemId,
    },
  });

  return result;
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
        content {
          type
          payload
        }
      }
    }
  }
`);

export async function useTodayHistory() {
  const client = getClient();
  const result = await client.query({
    query: TODAY_HISTORY,
    variables: {
      pagination: {
        page: 0,
        limit: 10,
      },
    },
  });

  return result;
}

const CREATE_TODAY = gql(`
mutation CreateToday {
  createToday {
    id
  }
}`);

export async function createToday() {
  const client = getClient();
  await client.mutate({
    mutation: CREATE_TODAY,
  });

  redirect(`/today`);
}
