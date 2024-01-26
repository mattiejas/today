import RouteGuard from "@/components/RouteGuard";
import { Input } from "@/components/ui/input";
import { Today } from "../../../__generated__/models";
import TodayBlock from "./TodayBlock";
import { createToday, useTodayHistory } from "@/app/actions";
import { Suspense } from "react";
import { Button } from "@/components/ui/button";
import { MdAddCircleOutline } from "react-icons/md";

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
      <div className="flex flex-col justify-between items-center gap-4 mb-4">
        <Input name="search" placeholder="Search" className="shadow-lg shadow-violet-700/20" />

        <form action={createToday}>
          <Button type="submit" className="shadow-lg rounded-full shadow-violet-700/20">
            <span className="mr-2">Create new</span>
            <MdAddCircleOutline className="text-xl" />
          </Button>
        </form>
      </div>
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
