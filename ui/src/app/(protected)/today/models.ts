import { TodayBlockContent as GraphqlTodayBlockContent } from "@/__generated__/graphql";

export interface Today {
  id: string;
  title: string;
  date: string;
  createdAt: string;
  updatedAt: string;
  items: TodayItem[];
}

export interface TodayItem {
  id: string;
  todayId: string;
  content: TodayBlockContent;
}

export interface TodayBlockContentBase {
  type: TodayBlockContentType;
}

export enum TodayBlockContentType {
  Text = "text",
  Todo = "todo",
}

export interface TodayBlockContentText {
  type: TodayBlockContentType.Text;
  payload: string;
}

export interface TodayBlockContentTodo {
  type: TodayBlockContentType.Todo;
  payload: {
    text: string;
    isCompleted: boolean;
  };
}

export type TodayBlockContent = TodayBlockContentBase & (TodayBlockContentText | TodayBlockContentTodo);

export function parseTodayItemContent(req: GraphqlTodayBlockContent): TodayBlockContent {
  return {
    type: req.type as TodayBlockContentType,
    payload: req.payload,
  };
}

export interface TodayBlockBaseProps {
  item: TodayItem;
  isLoading: boolean;
  onChange: (content: TodayBlockContent, id?: string) => void;
}
