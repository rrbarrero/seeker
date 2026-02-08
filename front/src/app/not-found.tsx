"use client";

import { ErrorState } from "@/components/shared/error-state";
import { FileQuestion } from "lucide-react";

export default function NotFound() {
  return (
    <ErrorState
      title="Page Not Found"
      description="The page you are looking for does not exist or has been moved."
      homeLink="/"
      fullPage={true}
      icon={FileQuestion}
    />
  );
}
