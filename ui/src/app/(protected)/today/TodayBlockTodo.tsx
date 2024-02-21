import { Checkbox } from '@/components/ui/checkbox'
import {
  type TodayBlockBaseProps,
  type TodayBlockContentTodo,
} from '@gql/models'

interface TodayBlockTodoProps {
  content: TodayBlockContentTodo
}

type Props = TodayBlockBaseProps & TodayBlockTodoProps

export default function TodayBlockTodo({
  item,
  isLoading,
  content,
  onChange,
  contentEditableId,
}: Readonly<Props>): JSX.Element {
  return (
    <div className="flex items-center gap-3" id={contentEditableId}>
      <Checkbox
        checked={content.payload.isCompleted}
        id={item.id}
        onCheckedChange={() => {
          if (isLoading) return

          onChange(
            {
              type: content.type,
              payload: {
                ...content.payload,
                isCompleted: !content.payload.isCompleted,
              },
            },
            item.id,
          )
        }}
      />
      <label htmlFor={item.id}>{content.payload.text}</label>
    </div>
  )
}
