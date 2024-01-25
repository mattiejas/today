import { getClient } from "@/lib/client";
import { gql } from "@apollo/client";

export default async function Today() {
  return <TodayContainer></TodayContainer>;
}

const query = gql`
  query GetHistory {
    history(pagination: { page: 0, limit: 10 }) {
      id
      title
      items {
        content
      }
    }
  }
`;

async function TodayContainer(): Promise<JSX.Element> {
  const { data, loading } = await getClient().query({ query });

  if (loading) {
    return <div>Loading...</div>;
  }

  return <>{JSON.stringify(data)}</>;
}
