"use client";

import { upsertTodayBlock } from "@/app/actions";
import TodayBlockText from "./TodayBlockText";
import TodayBlockTodo from "./TodayBlockTodo";
import { TodayBlockContent, TodayBlockContentType, TodayItem } from "../../../__generated__/models";
import { useState, useTransition } from "react";
import { useRouter } from "next/navigation";

interface TodayBlockProps {
  item: TodayItem;
}

export default function TodayBlock({ ...props }: Readonly<TodayBlockProps>): JSX.Element {
  const router = useRouter();
  const [item, setItem] = useState(props.item);
  const [isPending, startTransition] = useTransition();

  const onChange = (content: TodayBlockContent, id?: string) => {
    upsertTodayBlock(item.todayId, content, id).then((res) => {
      if (res.data?.upsertItem) {
        setItem(res.data?.upsertItem as TodayItem);
      }
    });
  };

  const getItemEl = () => {
    switch (item.content.type) {
      case TodayBlockContentType.Text:
        return <TodayBlockText item={item} content={item.content} onChange={onChange} isLoading={isPending} />;
      case TodayBlockContentType.Todo:
        return <TodayBlockTodo item={item} content={item.content} onChange={onChange} isLoading={isPending} />;
      default:
        return <div>Unknown block type</div>;
    }
  };

  return <div className="hover:bg-violet-100 rounded">{getItemEl()}</div>;
}
