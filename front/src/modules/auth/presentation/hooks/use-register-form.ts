"use client";

import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import { useRouter } from "next/navigation";
import { authService } from "../../composition-root";
import { registerSchema, type RegisterFormValues } from "../../domain/schema";
import { UiErrorHandler } from "@/shared/presentation/error-handler";

export function useRegisterForm() {
  const router = useRouter();

  const form = useForm<RegisterFormValues>({
    resolver: zodResolver(registerSchema),
    defaultValues: {
      email: "",
      password: "",
      confirmPassword: "",
    },
  });

  const onSubmit = async (data: RegisterFormValues) => {
    try {
      await authService.register(data);
      toast.success("Account created successfully", {
        description: "You can now log in with your credentials.",
      });
      router.push("/auth/login");
    } catch (error) {
      UiErrorHandler.handle(error, "There was a problem creating your account.");
    }
  };

  return {
    form,
    onSubmit: form.handleSubmit(onSubmit),
    isLoading: form.formState.isSubmitting,
  };
}
