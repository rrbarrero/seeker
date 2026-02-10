"use client";

import { useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";
import { Suspense } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { authService } from "@/modules/auth/composition-root";

type VerificationState = "loading" | "success" | "error" | "expired" | "no-token";

function VerifyEmailContent() {
  const searchParams = useSearchParams();
  const token = searchParams.get("token");
  const [state, setState] = useState<VerificationState>(() => (token ? "loading" : "no-token"));
  const [errorMessage, setErrorMessage] = useState("");

  useEffect(() => {
    if (!token) {
      return;
    }

    const verify = async () => {
      try {
        await authService.verifyEmail(token);
        setState("success");
      } catch (error) {
        if (error instanceof Error && error.message.includes("expired")) {
          setState("expired");
        } else {
          setState("error");
          setErrorMessage(error instanceof Error ? error.message : "An unexpected error occurred");
        }
      }
    };

    verify();
  }, [token]);

  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 p-4 dark:bg-gray-900">
      <Card className="mx-auto w-full max-w-md">
        {state === "loading" && (
          <>
            <CardHeader>
              <CardTitle className="text-center text-2xl font-bold">
                Verifying your email...
              </CardTitle>
              <CardDescription className="text-center">
                Please wait while we verify your email address.
              </CardDescription>
            </CardHeader>
            <CardContent className="flex justify-center py-8">
              <div className="border-primary h-8 w-8 animate-spin rounded-full border-4 border-t-transparent" />
            </CardContent>
          </>
        )}

        {state === "success" && (
          <>
            <CardHeader>
              <CardTitle className="text-center text-2xl font-bold">✅ Email Verified!</CardTitle>
              <CardDescription className="text-center">
                Your email address has been successfully verified. You can now access all features.
              </CardDescription>
            </CardHeader>
            <CardFooter className="flex justify-center">
              <Button asChild>
                <a href="/auth/login">Go to Login</a>
              </Button>
            </CardFooter>
          </>
        )}

        {state === "expired" && (
          <>
            <CardHeader>
              <CardTitle className="text-center text-2xl font-bold">⏰ Link Expired</CardTitle>
              <CardDescription className="text-center">
                This verification link has expired. Please request a new one.
              </CardDescription>
            </CardHeader>
            <CardFooter className="flex justify-center">
              <Button asChild variant="outline">
                <a href="/auth/login">Go to Login</a>
              </Button>
            </CardFooter>
          </>
        )}

        {state === "error" && (
          <>
            <CardHeader>
              <CardTitle className="text-center text-2xl font-bold">
                ❌ Verification Failed
              </CardTitle>
              <CardDescription className="text-center">
                {errorMessage || "We couldn't verify your email. Please try again."}
              </CardDescription>
            </CardHeader>
            <CardFooter className="flex justify-center">
              <Button asChild variant="outline">
                <a href="/auth/login">Go to Login</a>
              </Button>
            </CardFooter>
          </>
        )}

        {state === "no-token" && (
          <>
            <CardHeader>
              <CardTitle className="text-center text-2xl font-bold">⚠️ Invalid Link</CardTitle>
              <CardDescription className="text-center">
                No verification token was found. Please check the link in your email.
              </CardDescription>
            </CardHeader>
            <CardFooter className="flex justify-center">
              <Button asChild variant="outline">
                <a href="/auth/login">Go to Login</a>
              </Button>
            </CardFooter>
          </>
        )}
      </Card>
    </div>
  );
}

export default function VerifyEmailPage() {
  return (
    <Suspense
      fallback={
        <div className="flex min-h-screen items-center justify-center bg-gray-50 p-4 dark:bg-gray-900">
          <div className="border-primary h-8 w-8 animate-spin rounded-full border-4 border-t-transparent" />
        </div>
      }
    >
      <VerifyEmailContent />
    </Suspense>
  );
}
