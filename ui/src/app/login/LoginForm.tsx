import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import config from "@/lib/config";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";

export default function LoginForm(): JSX.Element {
  async function login(formData: FormData) {
    "use server";

    const username = formData.get("username");
    const password = formData.get("password");

    const res = await fetch(`${config.apiBaseUrl}/auth/login`, {
      method: "POST",
      body: JSON.stringify({ username, password }),
      headers: {
        "Content-Type": "application/json",
      },
    });

    if (res.ok) {
      cookies().set("token", await res.text());
      redirect("/today");
    }
  }

  return (
    <div className="flex flex-col justify-center h-screen w-full">
      <h1 className="text-3xl font-bold mb-6">Login</h1>
      <form action={login} className="flex flex-col gap-4">
        <Input name="username" placeholder="username" required />
        <Input name="password" placeholder="password" type="password" required />

        <Button type="submit">Login</Button>
      </form>
    </div>
  );
}
