import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import config from '@/lib/config'
import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'

interface RegisterFormProps {
  error?: string
}

export default function RegisterForm({
  error,
}: Readonly<RegisterFormProps>): JSX.Element {
  async function login(formData: FormData): Promise<void> {
    'use server'

    const username = formData.get('username')
    const password = formData.get('password')
    const confirmPassword = formData.get('repeat-password')
    const email = formData.get('email')

    if (password !== confirmPassword) {
      const error = 'The chosen passwords do not match. Please try again.'
      redirect(`/register?error=${error}`)
    }

    const res = await fetch(`${config.apiBaseUrl}/auth/register`, {
      method: 'POST',
      body: JSON.stringify({ username, password, email }),
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (res.ok) {
      cookies().set('token', await res.text())
      redirect('/today')
    }

    const data = await res.json()

    const error = encodeURIComponent(data.message)
    console.log(error)
    redirect(`/register?error=${error}`)
  }

  return (
    <div className="flex flex-col justify-center md:h-screen w-full py-20 md:pb-80">
      <h1 className="text-3xl font-bold mb-6">Register</h1>
      <form action={login} className="flex flex-col gap-4">
        <Input name="username" placeholder="username" required />
        <Input name="email" placeholder="email" type="email" required />
        <Input
          name="password"
          placeholder="password"
          type="password"
          required
        />
        <Input
          name="repeat-password"
          placeholder="confirm password"
          type="password"
          required
        />

        <Button type="submit">Register</Button>

        {error && (
          <div className="text-red-500/80 italic font-bold text-sm">
            * {error}
          </div>
        )}

        <hr className="my-6 w-full border-t-2 border-violet-300" />

        <div className="flex flex-col gap-4">
          <h2 className="text-xl font-bold">Already have an account?</h2>
          <Button asChild variant="secondary">
            <a href="/login">Login</a>
          </Button>
        </div>
      </form>
    </div>
  )
}
