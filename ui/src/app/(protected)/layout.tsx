import LogoutButton from '@/components/LogoutButton'
import { Toaster } from '@/components/ui/toaster'

interface LayoutProps {
  children: React.ReactNode
}

export default async function Layout({
  children,
}: Readonly<LayoutProps>): Promise<JSX.Element> {
  return (
    <div className="flex flex-col min-h-screen">
      <header className="bg-indigo-600 rounded-lg mt-4 mb-8 shadow-xl shadow-violet-700/20">
        <div className="flex items-center justify-between px-4 py-3">
          <div>
            <h1 className="text-2xl font-bold text-white pb-[2px]">Today.</h1>
          </div>

          <LogoutButton />
        </div>
      </header>
      <main className="flex-grow">
        <div>{children}</div>

        <Toaster />
      </main>
    </div>
  )
}
