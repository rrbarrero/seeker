"use client";

import { useEffect } from "react";
import { AlertTriangle, RefreshCw, Home } from "lucide-react";
import { Button } from "@/components/ui/button";
import Link from "next/link";

export default function Error({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error(error);
  }, [error]);

  return (
    <div className="flex min-h-[70vh] flex-col items-center justify-center p-6 text-center">
      <div className="mb-6 rounded-full bg-amber-100 p-4 dark:bg-amber-900/20">
        <AlertTriangle className="h-10 w-10 text-amber-600 dark:text-amber-500" />
      </div>

      <h1 className="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-gray-100">
        Something went wrong in the dashboard
      </h1>

      <p className="mb-8 max-w-md text-gray-600 dark:text-gray-400">
        {error.message ||
          "An unexpected error occurred while loading this section. Our team has been notified."}
      </p>

      <div className="flex flex-col gap-3 sm:flex-row">
        <Button onClick={() => reset()} className="flex items-center gap-2">
          <RefreshCw className="h-4 w-4" />
          Try again
        </Button>

        <Button variant="outline" asChild>
          <Link href="/" className="flex items-center gap-2">
            <Home className="h-4 w-4" />
            Go back home
          </Link>
        </Button>
      </div>

      {error.digest && <p className="mt-8 text-xs text-gray-400">Error ID: {error.digest}</p>}
    </div>
  );
}
