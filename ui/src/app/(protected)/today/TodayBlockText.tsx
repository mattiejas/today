import { TodayBlockBaseProps, TodayBlockContentText } from "./models";

interface TodayBlockTextProps {
  content: TodayBlockContentText;
}

type Props = TodayBlockBaseProps & TodayBlockTextProps;

export default function TodayBlockText({ content }: Readonly<Props>): JSX.Element {
  return <div>{content.payload}</div>;
}
