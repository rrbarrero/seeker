import { LoginForm } from "@/modules/auth/presentation/components/login-form";

export default function LoginPage() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 p-4 dark:bg-gray-900">
      <LoginForm />
    </div>
  );
}
