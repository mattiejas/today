import {Button} from "@/components/ui/button";
import {Input} from "@/components/ui/input";
import config from "@/lib/config";
import {cookies} from "next/headers";
import {redirect} from "next/navigation";

interface LoginFormProps {
    error?: string;
}

export default function LoginForm({ error }: Readonly<LoginFormProps>): JSX.Element {
    async function login(formData: FormData) {
        "use server";

        const username = formData.get("username");
        const password = formData.get("password");

        const res = await fetch(`${config.apiBaseUrl}/auth/login`, {
            method: "POST",
            body: JSON.stringify({username, password}),
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (res.ok) {
            cookies().set("token", await res.text());
            redirect("/today");
        }

        const data = await res.json();
        const error = encodeURIComponent(data.message);

        redirect(`/login?error=${error}`);
    }

    return (
        <div className="flex flex-col justify-center md:h-screen w-full py-20 md:pb-80">
            <h1 className="text-3xl font-bold mb-6">Login</h1>
            <form action={login} className="flex flex-col gap-4">
                <Input name="username" placeholder="username" required/>
                <Input name="password" placeholder="password" type="password" required/>

                <Button type="submit">Login</Button>
                {error && <div className="text-red-500/80 italic font-bold text-sm">* {error}</div>}
            </form>

            <hr className="my-6 w-full border-t-2 border-violet-300"/>

            <div className="flex flex-col gap-4">
                <h2 className="text-xl font-bold">Don't have an account?</h2>
                <Button asChild variant="secondary">
                    <a href="/register">Register</a>
                </Button>
            </div>
        </div>
    );
}
