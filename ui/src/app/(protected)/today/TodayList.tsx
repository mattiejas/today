'use client'
import { createToday } from '@/app/actions'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { MdAddCircleOutline } from 'react-icons/md'
import { type GetHistoryQuery } from '@gql/graphql'
import { type Today } from '@gql/models'
import { createApolloClient } from '@/lib/apollo-client'
import { ApolloProvider, useApolloClient } from '@apollo/client'
import TodayListItem from '@/app/(protected)/today/TodayListItem'
import ContentEditable from '@/components/ContentEditable'

interface TodayListProps {
  data: GetHistoryQuery
}

export default function TodayListContainer({
  data,
}: TodayListProps): JSX.Element {
  const client = createApolloClient()

  return (
    <ApolloProvider client={client}>
      <TodayList data={data} />
    </ApolloProvider>
  )
}

export function TodayList({ data }: TodayListProps): JSX.Element {
  return (
    <div>
      <div className="flex flex-col justify-between items-center gap-4 mb-8">
        <Input
          name="search"
          placeholder="Search"
          className="shadow-lg shadow-violet-700/20"
        />

        <form action={createToday}>
          <Button
            type="submit"
            className="shadow-lg rounded-full shadow-violet-700/20"
          >
            <span className="mr-2">Create new</span>
            <MdAddCircleOutline className="text-xl" />
          </Button>
        </form>
      </div>

      {data.history.map((day) => {
        return <TodayListItem key={day.id} day={day as Today} />
      })}
    </div>
  )
}
