import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import { useRouter } from "next/navigation";
import { authService } from "../../composition-root";
import { loginSchema, type LoginFormValues } from "../../domain/schema";

export function useLoginForm() {
  const router = useRouter();

  const form = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const onSubmit = async (data: LoginFormValues) => {
    try {
      await authService.login(data);
      toast.success("Login successful", {
        description: "Welcome back!",
      });
      router.push("/dashboard");
    } catch (error) {
      toast.error("Login failed", {
        description: "Please check your email and password.",
      });
      console.error(error);
    }
  };

  return {
    form,
    onSubmit: form.handleSubmit(onSubmit),
    isLoading: form.formState.isSubmitting,
  };
}
