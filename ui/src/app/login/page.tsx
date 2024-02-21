import LoginForm from './LoginForm'

interface LoginProps {
  searchParams: Record<string, string | string[] | undefined>
}

export default async function Login({
  searchParams,
}: Readonly<LoginProps>): Promise<JSX.Element> {
  const error = searchParams.error as string | undefined
  return <LoginForm error={error} />
}
