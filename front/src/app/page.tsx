import Link from "next/link";
import { Button } from "@/components/ui/button";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-zinc-50 px-4 py-8 font-sans dark:bg-black">
      <div className="w-full max-w-3xl space-y-8 text-center">
        <h1 className="text-5xl font-extrabold tracking-tight text-zinc-900 sm:text-6xl dark:text-zinc-50">
          Best <span className="text-primary">Seeker</span>
        </h1>
        <p className="mx-auto max-w-2xl text-xl text-zinc-600 dark:text-zinc-400">
          The ultimate tool to track your job applications and land your dream job. Simple, safe,
          and efficient.
        </p>
        <div className="flex flex-col items-center justify-center gap-4 pt-4 sm:flex-row">
          <Link href="/auth/login">
            <Button size="lg" className="w-full text-lg sm:w-[200px]">
              Get Started
            </Button>
          </Link>
          <Link href="/auth/register">
            <Button size="lg" variant="outline" className="w-full text-lg sm:w-[200px]">
              Sign Up
            </Button>
          </Link>
        </div>
      </div>
      <footer className="mt-20 text-sm text-zinc-500">
        © {new Date().getFullYear()} Best Seeker. All rights reserved.
      </footer>
    </div>
  );
}
