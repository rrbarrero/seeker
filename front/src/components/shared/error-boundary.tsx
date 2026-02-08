"use client";

import { Component, type ErrorInfo, type ReactNode } from "react";
import { AlertCircle, RefreshCcw } from "lucide-react";
import { Button } from "@/components/ui/button";

interface Props {
  children?: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("Uncaught error:", error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="flex min-h-[400px] w-full flex-col items-center justify-center space-y-4 rounded-lg border border-red-200 bg-red-50 p-8 text-center dark:border-red-900/50 dark:bg-red-900/10">
          <div className="rounded-full bg-red-100 p-3 dark:bg-red-900/20">
            <AlertCircle className="h-8 w-8 text-red-600 dark:text-red-400" />
          </div>
          <div className="space-y-2">
            <h2 className="text-xl font-semibold text-red-900 dark:text-red-100">
              Something went wrong
            </h2>
            <p className="max-w-[400px] text-sm text-red-700 dark:text-red-300">
              {this.state.error?.message || "An unexpected error occurred in the UI."}
            </p>
          </div>
          <Button
            variant="outline"
            onClick={() => this.setState({ hasError: false })}
            className="flex items-center gap-2"
          >
            <RefreshCcw className="h-4 w-4" />
            Try again
          </Button>
        </div>
      );
    }

    return this.props.children;
  }
}
