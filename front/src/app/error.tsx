"use client";

import { useEffect } from "react";
import { ErrorState } from "@/components/shared/error-state";

export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    // Log the error to an error reporting service
    console.error("Global Error:", error);
  }, [error]);

  return (
    <ErrorState
      title="Something went wrong"
      description="We apologize for the inconvenience. An unexpected error occurred at the application level."
      error={error}
      reset={reset}
      fullPage={true}
      homeLink="/"
    />
  );
}
