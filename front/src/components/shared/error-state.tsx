"use client";

import type { LucideIcon } from "lucide-react";
import { AlertCircle, RefreshCcw, Home } from "lucide-react";
import { Button } from "@/components/ui/button";
import Link from "next/link";
import { cn } from "@/lib/utils";

interface ErrorStateProps {
  title: string;
  description: string;
  error?: Error & { digest?: string };
  reset?: () => void;
  homeLink?: string;
  icon?: LucideIcon;
  fullPage?: boolean;
  className?: string;
}

export function ErrorState({
  title,
  description,
  error,
  reset,
  homeLink = "/",
  icon: Icon = AlertCircle,
  fullPage = false,
  className,
}: ErrorStateProps) {
  return (
    <div
      className={cn(
        "flex flex-col items-center justify-center bg-white p-6 text-center dark:bg-slate-950",
        fullPage
          ? "min-h-screen"
          : "min-h-[400px] w-full rounded-lg border border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-900/50",
        className,
      )}
    >
      <div className="mb-6 rounded-full bg-red-100 p-4 dark:bg-red-900/20">
        <Icon className="h-12 w-12 text-red-600 dark:text-red-500" />
      </div>

      <h2 className="mb-4 text-2xl font-bold tracking-tight text-slate-900 dark:text-slate-50">
        {title}
      </h2>

      <p className="mb-8 max-w-md text-slate-600 dark:text-slate-400">{description}</p>

      {error?.message && (
        <div className="mb-8 w-full max-w-xl overflow-auto rounded-md bg-slate-100 p-4 text-left dark:bg-slate-900">
          <code className="block font-mono text-xs text-red-600 dark:text-red-400">
            {error.name}: {error.message}
          </code>
          {error.digest && (
            <p className="mt-2 font-mono text-xs text-slate-400">Error ID: {error.digest}</p>
          )}
        </div>
      )}

      <div className="flex flex-col gap-4 sm:flex-row">
        {reset && (
          <Button onClick={reset} className="flex items-center gap-2 px-6">
            <RefreshCcw className="h-4 w-4" />
            Try again
          </Button>
        )}

        {homeLink && (
          <Button variant="outline" asChild className="flex items-center gap-2 px-6">
            <Link href={homeLink}>
              <Home className="h-4 w-4" />
              Back to Home
            </Link>
          </Button>
        )}
      </div>
    </div>
  );
}
