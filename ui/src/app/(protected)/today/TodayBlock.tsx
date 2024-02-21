'use client'

import { upsertTodayBlock } from '@/app/actions'
import TodayBlockText from './TodayBlockText'
import TodayBlockTodo from './TodayBlockTodo'
import {
  type TodayBlockBaseProps,
  type TodayBlockContent,
  TodayBlockContentType,
  type TodayItem,
} from '@gql/models'
import { useState } from 'react'

interface TodayBlockProps {
  item: TodayItem
  selectedItemId: string | undefined
  insert: (insertAt: number) => void
  onItemFocus: (id: string) => void
}

export default function TodayBlock({
  ...props
}: Readonly<TodayBlockProps>): JSX.Element {
  const [item, setItem] = useState(props.item)

  const onChange = (content: TodayBlockContent, id?: string): void => {
    void upsertTodayBlock(item.todayId, content, id).then((res) => {
      if (res.data?.upsertItem != null) {
        setItem(res.data?.upsertItem as TodayItem)
      }
    })
  }

  const commonProps: TodayBlockBaseProps = {
    onChange,
    insert: props.insert,
    item,
    isLoading: false,
    contentEditableId: getContentEditableId(item),
    onItemFocus: props.onItemFocus,
    selectedItemId: props.selectedItemId,
  }

  const getItemEl = (): JSX.Element => {
    switch (item.content.type) {
      case TodayBlockContentType.Text:
        return <TodayBlockText {...commonProps} content={item.content} />
      case TodayBlockContentType.Todo:
        return <TodayBlockTodo {...commonProps} content={item.content} />
      default:
        return <div>Unknown block type</div>
    }
  }

  return <div className="hover:bg-violet-100 rounded">{getItemEl()}</div>
}

export function getContentEditableId(item: TodayItem): string {
  return `item-${item.todayId}-${item.id}`
}
