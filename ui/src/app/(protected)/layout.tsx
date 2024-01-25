import LogoutButton from "@/components/LogoutButton";

interface LayoutProps {
  children: React.ReactNode;
}

export default async function Layout({ children }: Readonly<LayoutProps>) {
  return (
    <div className="flex flex-col min-h-screen">
      <header className="bg-indigo-600 rounded-lg mt-4">
        <div className="flex items-center justify-between px-4 py-3">
          <div>
            <h1 className="text-xl font-black text-white">Today</h1>
          </div>

          <LogoutButton />
        </div>
      </header>
      <main className="flex-grow">
        <div className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">{children}</div>
      </main>
    </div>
  );
}
