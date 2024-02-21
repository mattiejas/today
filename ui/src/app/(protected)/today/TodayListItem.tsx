'use client'

import TodayBlock from '@/app/(protected)/today/TodayBlock'
import { type Today, TodayBlockContentType } from '@gql/models'
import { type ReactNode, useEffect, useState } from 'react'
import { upsertTodayBlock } from '@/app/actions'
import { useQuery } from '@apollo/client'
import { gql } from '@gql'

interface TodayProps {
  day: Today
}

const GET_TODAY_QUERY = gql(`
  query GetTodayById($id: Uuid!) {
    todayById(todayId: $id) {
      id
      title
      date
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

export default function TodayListItem({
  day: initialDay,
}: Readonly<TodayProps>): ReactNode {
  const [day, setDay] = useState(initialDay)
  const [selectedItemId, setSelectedItemId] = useState<string | undefined>(
    undefined,
  )

  if (day.id === undefined) {
    console.error('day.id is undefined')
    return null
  }

  const { data, refetch } = useQuery(GET_TODAY_QUERY, {
    variables: { id: day.id },
  })

  useEffect(() => {
    if (data !== undefined) {
      setDay(data.todayById as Today)
    }
  }, [data])

  useEffect(() => {
    if (selectedItemId !== undefined && day.items.length > 0) {
      const selectedItem = day.items.find((i) => i.id === selectedItemId)

      console.log(selectedItem)
    }
  }, [selectedItemId])

  const insert = async (insertAt: number): Promise<void> => {
    const { data } = await upsertTodayBlock(
      day.id,
      { type: TodayBlockContentType.Text, payload: '' },
      undefined,
      insertAt,
    )

    await refetch({ id: day.id })

    // get the item that was just inserted
    if (data?.upsertItem !== undefined) {
      setSelectedItemId(data.upsertItem.id as string)
    }
  }

  const onItemFocus = (id: string): void => {
    setSelectedItemId(id)
  }

  return (
    <div className="flex flex-col gap-4 mb-12 bg-white p-6 rounded-lg shadow-md shadow-violet-700/20">
      <h2 className="text-3xl font-extrabold">{day.title}</h2>
      <hr className="border-violet-300 shadow-sm shadow-violet-500/20" />

      <div className="flex flex-col gap-2">
        {day.items.map((item) => {
          return (
            <TodayBlock
              key={item.id}
              item={item}
              selectedItemId={selectedItemId}
              insert={insert}
              onItemFocus={onItemFocus}
            />
          )
        })}
      </div>
    </div>
  )
}
