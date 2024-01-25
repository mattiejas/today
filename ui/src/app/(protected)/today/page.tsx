import { gql } from "@gql";
import RouteGuard from "@/components/RouteGuard";
import { getClient } from "@/lib/client";
import { Input } from "@/components/ui/input";

export default async function Page() {
  return (
    <RouteGuard>
      <TodayContainer />
    </RouteGuard>
  );
}

const query = gql(/* GraphQL */ `
  query GetHistory {
    history(pagination: { page: 0, limit: 10 }) {
      id
      title
      date
      items {
        id
        content
      }
    }
  }
`);

async function TodayContainer(): Promise<JSX.Element> {
  const { data, loading } = await getClient().query({ query });

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <>
      <Input name="search" placeholder="Search" className="mb-12 shadow-lg shadow-violet-700/20" />
      {data.history.map((day: any) => {
        return <Today key={day.id} day={day} />;
      })}
    </>
  );
}

interface TodayProps {
  day: {
    id: string;
    title: string;
    date: string;
    items: { id?: string; content: string }[];
  };
}

async function Today({ day }: Readonly<TodayProps>): Promise<JSX.Element> {
  return (
    <div className="flex flex-col gap-4 mb-12 bg-white p-6 rounded-lg shadow-md shadow-violet-700/20">
      <h2 className="text-3xl font-extrabold">{day.title}</h2>
      <hr className="border-violet-300 shadow-sm shadow-violet-500/20" />

      <div className="flex flex-col gap-4">
        {day.items.map((item: any) => {
          return <div key={item.id}>{item.content}</div>;
        })}
      </div>
    </div>
  );
}
