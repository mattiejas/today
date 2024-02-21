import RegisterForm from './RegisterForm'

interface RegisterProps {
  searchParams: Record<string, string | string[] | undefined>
}

export default async function Register({
  searchParams,
}: Readonly<RegisterProps>): Promise<JSX.Element> {
  const error = searchParams.error as string | undefined
  return <RegisterForm error={error} />
}
