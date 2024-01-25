"use client";

import { upsertTodayBlock } from "@/app/actions";
import TodayBlockText from "./TodayBlockText";
import TodayBlockTodo from "./TodayBlockTodo";
import { TodayBlockContent, TodayBlockContentType, TodayItem } from "./models";
import { useTransition } from "react";
import { useRouter } from "next/navigation";

interface TodayBlockProps {
  item: TodayItem;
}

export default function TodayBlock({ item }: Readonly<TodayBlockProps>): JSX.Element {
  const router = useRouter();
  const [isPending, startTransition] = useTransition();

  const onChange = (content: TodayBlockContent, id?: string) => {
    upsertTodayBlock(item.todayId, content, id);

    console.log("refreshing");

    startTransition(() => {
      router.refresh();
    });
  };

  switch (item.content.type) {
    case TodayBlockContentType.Text:
      return <TodayBlockText item={item} content={item.content} onChange={onChange} isLoading={isPending} />;
    case TodayBlockContentType.Todo:
      return <TodayBlockTodo item={item} content={item.content} onChange={onChange} isLoading={isPending} />;
    default:
      return <div>Unknown block type</div>;
  }
}
