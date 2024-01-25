import RouteGuard from "@/components/RouteGuard";
import { Input } from "@/components/ui/input";
import { Today } from "./models";
import TodayBlock from "./TodayBlock";
import { useTodayHistory } from "@/app/actions";
import { Suspense } from "react";

export default async function Page() {
  return (
    <RouteGuard>
      <TodayContainer />
    </RouteGuard>
  );
}

async function TodayContainer(): Promise<JSX.Element> {
  const { data } = await useTodayHistory();

  return (
    <>
      <Input name="search" placeholder="Search" className="mb-12 shadow-lg shadow-violet-700/20" />
      {data?.history.map((day) => {
        return <Today key={day.id} day={day as Today} />;
      })}
    </>
  );
}

interface TodayProps {
  day: Today;
}

async function Today({ day }: Readonly<TodayProps>): Promise<JSX.Element> {
  return (
    <div className="flex flex-col gap-4 mb-12 bg-white p-6 rounded-lg shadow-md shadow-violet-700/20">
      <h2 className="text-3xl font-extrabold">{day.title}</h2>
      <hr className="border-violet-300 shadow-sm shadow-violet-500/20" />

      <div className="flex flex-col gap-4">
        {day.items.map((item: any) => {
          return (
            <Suspense key={item.id} fallback={<div>Loading...</div>}>
              <TodayBlock item={item} />
            </Suspense>
          );
        })}
      </div>
    </div>
  );
}
