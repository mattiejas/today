import { Checkbox } from "@/components/ui/checkbox";
import { TodayBlockBaseProps, TodayBlockContentTodo } from "./models";

interface TodayBlockTodoProps {
  content: TodayBlockContentTodo;
}

type Props = TodayBlockBaseProps & TodayBlockTodoProps;

export default function TodayBlockTodo({ item, isLoading, content, onChange }: Readonly<Props>): JSX.Element {
  return (
    <div className="flex items-center gap-3">
      <Checkbox
        checked={content.payload.isCompleted}
        id={item.id}
        onCheckedChange={() => {
          if (isLoading) return;

          onChange(
            {
              type: content.type,
              payload: {
                ...content.payload,
                isCompleted: !content.payload.isCompleted,
              },
            },
            item.id
          );
        }}
      />
      <label htmlFor={item.id}>{content.payload.text}</label>
    </div>
  );
}
