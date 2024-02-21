'use server'

import config from '@/lib/config'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'

async function isAuthenticated(): Promise<boolean> {
  const token = cookies().get('token')?.value

  if (token === undefined) {
    return false
  }

  const res = await fetch(`${config.apiBaseUrl}/auth/me`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  })

  return res.ok
}

interface RouteGuardProps {
  children: React.ReactNode
}

export default async function RouteGuard({
  children,
}: Readonly<RouteGuardProps>): Promise<JSX.Element> {
  if (await isAuthenticated()) {
    return <>{children}</>
  }

  return redirect('/login')
}
