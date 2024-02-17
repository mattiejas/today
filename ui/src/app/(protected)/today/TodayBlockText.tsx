import ContentEditable from "@/components/ContentEditable";
import { TodayBlockBaseProps, TodayBlockContentText } from "../../../__generated__/models";

interface TodayBlockTextProps {
  content: TodayBlockContentText;
}

type Props = TodayBlockBaseProps & TodayBlockTextProps;

export default function TodayBlockText({ content, onChange, item }: Readonly<Props>): JSX.Element {
  const onEnter = (e: React.KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Enter") {
      e.preventDefault();
      e.currentTarget.blur();
    }
  };

  return (
    <ContentEditable
      onKeyDown={onEnter}
      onChange={(e) => onChange({ type: content.type, payload: (e.target as HTMLDivElement).innerHTML }, item.id)}
      value={content.payload}
    />
  );
}
