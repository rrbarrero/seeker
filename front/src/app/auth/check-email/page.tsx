import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";

export default function CheckEmailPage() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 p-4 dark:bg-gray-900">
      <Card className="mx-auto w-full max-w-md">
        <CardHeader>
          <CardTitle className="text-center text-2xl font-bold">📧 Check your email</CardTitle>
          <CardDescription className="text-center text-base leading-relaxed">
            We&apos;ve sent a verification link to your email address. Please click the link to
            activate your account.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-3 text-center">
          <p className="text-muted-foreground text-sm">
            Didn&apos;t receive the email? Check your spam folder or try registering again.
          </p>
          <p className="text-muted-foreground text-sm">
            The verification link will expire in a few hours.
          </p>
        </CardContent>
        <CardFooter className="flex justify-center gap-3">
          <Button asChild variant="outline">
            <a href="/auth/login">Go to Login</a>
          </Button>
          <Button asChild variant="ghost">
            <a href="/auth/register">Register again</a>
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}
