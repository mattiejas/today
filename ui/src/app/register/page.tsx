import RegisterForm from "./RegisterForm";

interface RegisterProps {
    searchParams: { [key: string]: string | string[] | undefined }
}

export default async function Register({ searchParams }: Readonly<RegisterProps>) {
    const error = searchParams.error as string | undefined;
    return <RegisterForm error={error}/>;
}
