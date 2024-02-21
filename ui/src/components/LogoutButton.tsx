import { logout } from '@/app/actions'
import { Button } from './ui/button'

export default async function LogoutButton(): Promise<JSX.Element> {
  return (
    <form action={logout}>
      <Button type="submit" variant="default">
        Logout
      </Button>
    </form>
  )
}
