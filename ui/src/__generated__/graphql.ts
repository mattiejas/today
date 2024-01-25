/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /** A scalar that can represent any JSON value. */
  JSON: { input: any; output: any; }
  /**
   * ISO 8601 calendar date without timezone.
   * Format: %Y-%m-%d
   *
   * # Examples
   *
   * * `1994-11-13`
   * * `2000-02-24`
   */
  NaiveDate: { input: any; output: any; }
  /**
   * ISO 8601 combined date and time without timezone.
   *
   * # Examples
   *
   * * `2015-07-01T08:59:60.123`,
   */
  NaiveDateTime: { input: any; output: any; }
  Uuid: { input: any; output: any; }
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  createToday: Today;
  deleteItem: Scalars['Boolean']['output'];
  upsertItem: TodayItem;
};


export type MutationRootDeleteItemArgs = {
  todayItemId: Scalars['Uuid']['input'];
};


export type MutationRootUpsertItemArgs = {
  content: Scalars['JSON']['input'];
  todayId: Scalars['Uuid']['input'];
  todayItemId?: InputMaybe<Scalars['Uuid']['input']>;
};

export type Pagination = {
  limit: Scalars['Int']['input'];
  page: Scalars['Int']['input'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  history: Array<Today>;
  sample: Scalars['String']['output'];
  today?: Maybe<Today>;
};


export type QueryRootHistoryArgs = {
  pagination: Pagination;
};

export type Today = {
  __typename?: 'Today';
  createdAt: Scalars['NaiveDateTime']['output'];
  date: Scalars['NaiveDate']['output'];
  id: Scalars['Uuid']['output'];
  items: Array<TodayItem>;
  title: Scalars['String']['output'];
  updatedAt: Scalars['NaiveDateTime']['output'];
  userId: Scalars['String']['output'];
};

export type TodayItem = {
  __typename?: 'TodayItem';
  content: Scalars['String']['output'];
  createdAt: Scalars['NaiveDateTime']['output'];
  id: Scalars['Uuid']['output'];
  todayId: Scalars['Uuid']['output'];
  updatedAt: Scalars['NaiveDateTime']['output'];
};

export type GetHistoryQueryVariables = Exact<{ [key: string]: never; }>;


export type GetHistoryQuery = { __typename?: 'QueryRoot', history: Array<{ __typename?: 'Today', id: any, title: string, date: any, items: Array<{ __typename?: 'TodayItem', id: any, content: string }> }> };


export const GetHistoryDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetHistory"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"history"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"page"},"value":{"kind":"IntValue","value":"0"}},{"kind":"ObjectField","name":{"kind":"Name","value":"limit"},"value":{"kind":"IntValue","value":"10"}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"date"}},{"kind":"Field","name":{"kind":"Name","value":"items"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"content"}}]}}]}}]}}]} as unknown as DocumentNode<GetHistoryQuery, GetHistoryQueryVariables>;