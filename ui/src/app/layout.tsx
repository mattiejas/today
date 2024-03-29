import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import '../styles/globals.css'
import { cn } from '@/lib/utils'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Today',
  description: 'A simple journaling app',
  icons: [
    {
      rel: 'icon',
      url: '/icon.svg',
    },
  ],
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>): JSX.Element {
  return (
    <html lang="en">
      <body
        className={cn(inter.className, 'bg-violet-100 max-w-2xl mx-auto px-4')}
      >
        {children}
      </body>
    </html>
  )
}
