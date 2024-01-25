import ContentEditable from "@/components/ContentEditable";
import { TodayBlockBaseProps, TodayBlockContentText } from "./models";

interface TodayBlockTextProps {
  content: TodayBlockContentText;
}

type Props = TodayBlockBaseProps & TodayBlockTextProps;

export default function TodayBlockText({ content, onChange, item }: Readonly<Props>): JSX.Element {
  return (
    <ContentEditable onChange={(e) => onChange({ type: content.type, payload: e.target.innerText }, item.id)} value={content.payload} />
  );
}
