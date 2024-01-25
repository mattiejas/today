"use client";
import { useAppSelector } from "./hooks";
import { useRouter } from "next/navigation";
import StoreProvider from "./StoreProvider";
import { useEffect } from "react";

export default function Home() {
  return (
    <StoreProvider>
      <RedirectIfNotLoggedIn>Hello</RedirectIfNotLoggedIn>
    </StoreProvider>
  );
}

function RedirectIfNotLoggedIn({ children }: Readonly<{ children: React.ReactNode }>) {
  const isLoggedIn = useAppSelector((state) => state.auth.isAuthenticated);
  const router = useRouter();

  useEffect(() => {
    if (!isLoggedIn) {
      router.push("/login");
    }
  }, [isLoggedIn, router]);

  if (!isLoggedIn) {
    return null;
  }

  return <>{children}</>;
}
