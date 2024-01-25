"use client";

import { Button } from "@/components/ui/button";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import z from "zod";
import { useAppDispatch } from "../hooks";
import { login } from "@/features/auth/authSlice";
import { useRouter } from "next/navigation";

const loginFormSchema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
});

export default function LoginForm(): JSX.Element {
  const dispatch = useAppDispatch();
  const router = useRouter();

  const form = useForm({
    resolver: zodResolver(loginFormSchema),
    defaultValues: {
      username: "",
      password: "",
    },
  });

  const onSubmit = async (data: z.infer<typeof loginFormSchema>) => {
    dispatch(
      login({
        username: data.username,
        password: data.password,
      })
    ).then(() => {
      form.reset();
      router.push("/today");
    });
  };

  return (
    <Form {...form}>
      <h1 className="text-3xl font-bold mb-6">Login</h1>
      <form onSubmit={form.handleSubmit(onSubmit)} className="flex flex-col space-y-4">
        <FormField
          control={form.control}
          name="username"
          render={({ field }) => (
            <FormItem>
              <FormLabel htmlFor={field.name}>Username</FormLabel>
              <FormControl>
                <Input {...field} placeholder="johndoe" />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          name="password"
          control={form.control}
          render={({ field }) => (
            <FormItem>
              <FormLabel htmlFor={field.name}>Password</FormLabel>
              <FormControl>
                <Input {...field} type="password" placeholder="********" />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <Button type="submit">Login</Button>
      </form>
    </Form>
  );
}
