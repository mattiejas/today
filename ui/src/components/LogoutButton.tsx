import { logout } from "@/app/actions";
import { Button } from "./ui/button";

export default async function LogoutButton() {
  return (
    <form action={logout}>
      <Button type="submit" variant="default">
        Logout
      </Button>
    </form>
  );
}
