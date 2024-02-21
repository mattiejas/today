import LoginForm from "./LoginForm";

export default async function Login({ searchParams }: Readonly<{ searchParams: { [key: string]: string | string[] | undefined } }>) {
  return <LoginForm error={searchParams.error} />;
}
