import RouteGuard from '@/components/RouteGuard'
import TodayList from '@/app/(protected)/today/TodayList'
import { useTodayHistory } from '@/app/actions'

export default async function Page(): Promise<JSX.Element> {
  const { data } = await useTodayHistory()

  if (data === null || data === undefined) {
    return <div>Loading...</div>
  }

  return (
    <RouteGuard>
      <TodayList data={data} />
    </RouteGuard>
  )
}
