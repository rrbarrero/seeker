import { RegisterForm } from "@/modules/auth/presentation/components/register-form";

export default function RegisterPage() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 p-4 dark:bg-gray-900">
      <RegisterForm />
    </div>
  );
}
