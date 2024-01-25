"use client";

import StoreProvider from "../StoreProvider";
import LoginForm from "./LoginForm";

export default function Login(): JSX.Element {
  return (
    <StoreProvider>
      <LoginForm />
    </StoreProvider>
  );
}
