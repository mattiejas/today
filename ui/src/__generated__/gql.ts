/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  query GetTodayById($id: Uuid!) {\n    todayById(todayId: $id) {\n      id\n      title\n      date\n      items {\n        id\n        todayId\n        sortOrder\n        content {\n          type\n          payload\n        }\n      }\n    }\n  }\n": types.GetTodayByIdDocument,
    "\n  mutation UpsertItem(\n    $todayId: Uuid!\n    $content: JSON!\n    $todayItemId: Uuid\n    $insertAt: Int\n  ) {\n    upsertItem(\n      todayId: $todayId\n      content: $content\n      todayItemId: $todayItemId\n      insertAt: $insertAt\n    ) {\n      id\n      todayId\n      sortOrder\n      content {\n        type\n        payload\n      }\n    }\n  }\n": types.UpsertItemDocument,
    "\n  query GetHistory($pagination: Pagination!) {\n    history(pagination: $pagination) {\n      id\n      title\n      date\n      createdAt\n      updatedAt\n      items {\n        id\n        todayId\n        sortOrder\n        content {\n          type\n          payload\n        }\n      }\n    }\n  }\n": types.GetHistoryDocument,
    "\nmutation CreateToday {\n  createToday {\n    id\n  }\n}": types.CreateTodayDocument,
};

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = gql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function gql(source: string): unknown;

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  query GetTodayById($id: Uuid!) {\n    todayById(todayId: $id) {\n      id\n      title\n      date\n      items {\n        id\n        todayId\n        sortOrder\n        content {\n          type\n          payload\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query GetTodayById($id: Uuid!) {\n    todayById(todayId: $id) {\n      id\n      title\n      date\n      items {\n        id\n        todayId\n        sortOrder\n        content {\n          type\n          payload\n        }\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation UpsertItem(\n    $todayId: Uuid!\n    $content: JSON!\n    $todayItemId: Uuid\n    $insertAt: Int\n  ) {\n    upsertItem(\n      todayId: $todayId\n      content: $content\n      todayItemId: $todayItemId\n      insertAt: $insertAt\n    ) {\n      id\n      todayId\n      sortOrder\n      content {\n        type\n        payload\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation UpsertItem(\n    $todayId: Uuid!\n    $content: JSON!\n    $todayItemId: Uuid\n    $insertAt: Int\n  ) {\n    upsertItem(\n      todayId: $todayId\n      content: $content\n      todayItemId: $todayItemId\n      insertAt: $insertAt\n    ) {\n      id\n      todayId\n      sortOrder\n      content {\n        type\n        payload\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  query GetHistory($pagination: Pagination!) {\n    history(pagination: $pagination) {\n      id\n      title\n      date\n      createdAt\n      updatedAt\n      items {\n        id\n        todayId\n        sortOrder\n        content {\n          type\n          payload\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  query GetHistory($pagination: Pagination!) {\n    history(pagination: $pagination) {\n      id\n      title\n      date\n      createdAt\n      updatedAt\n      items {\n        id\n        todayId\n        sortOrder\n        content {\n          type\n          payload\n        }\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\nmutation CreateToday {\n  createToday {\n    id\n  }\n}"): (typeof documents)["\nmutation CreateToday {\n  createToday {\n    id\n  }\n}"];

export function gql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;