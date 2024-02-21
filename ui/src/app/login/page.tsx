import LoginForm from "./LoginForm";

interface LoginProps {
  searchParams: { [key: string]: string | string[] | undefined }
}

export default async function Login({ searchParams }: Readonly<LoginProps>) {
  const error = searchParams.error as string | undefined;
  return <LoginForm error={error} />;
}
