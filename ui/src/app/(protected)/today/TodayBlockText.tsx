import ContentEditable from '@/components/ContentEditable'
import {
  type TodayBlockBaseProps,
  type TodayBlockContentText,
} from '@gql/models'
import { useEffect, useRef, useState } from 'react'

interface TodayBlockTextProps {
  content: TodayBlockContentText
}

type Props = TodayBlockBaseProps & TodayBlockTextProps

export default function TodayBlockText({
  content,
  onChange,
  onItemFocus,
  insert,
  item,
  selectedItemId,
  contentEditableId,
}: Readonly<Props>): JSX.Element {
  const [ref, setRef] = useState<HTMLDivElement | null>(null)

  useEffect(() => {
    if (ref !== null && item.id === selectedItemId) {
      ref.focus()
    }
  }, [ref, selectedItemId])

  const onEnter = (e: React.KeyboardEvent<HTMLDivElement>): void => {
    if (e.key === 'Enter') {
      e.preventDefault()
      insert(item.sortOrder + 1)
    }
  }

  return (
    <ContentEditable
      onRefTarget={setRef}
      id={contentEditableId}
      onKeyDown={onEnter}
      onFocus={() => onItemFocus(item.id)}
      onChange={(e) => {
        onChange(
          {
            type: content.type,
            payload: (e.target as HTMLDivElement).innerHTML,
          },
          item.id,
        )
      }}
      value={content.payload}
    />
  )
}
