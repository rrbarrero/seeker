import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import { UiErrorHandler } from "@/shared/presentation/error-handler";
import { positionService } from "../../composition-root";
import {
  createPositionFormSchema,
  type CreatePositionFormValues,
} from "./use-create-position-form";
import type { PositionProps, PositionStatus } from "../../domain/position";

export type UpdatePositionFormValues = CreatePositionFormValues;

interface UseUpdatePositionFormProps {
  position: PositionProps;
  onSuccess?: () => void;
}

const toDateInputValue = (value: string) => {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }
  return date.toISOString().split("T")[0];
};

export function useUpdatePositionForm({ position, onSuccess }: UseUpdatePositionFormProps) {
  const form = useForm<UpdatePositionFormValues>({
    resolver: zodResolver(createPositionFormSchema),
    defaultValues: {
      company: position.company,
      roleTitle: position.roleTitle,
      description: position.description,
      appliedOn: toDateInputValue(position.appliedOn),
      url: position.url,
      status: position.status,
    },
  });

  const onSubmit = async (values: UpdatePositionFormValues) => {
    try {
      const rfcDate = new Date(values.appliedOn).toUTCString();

      await positionService.updatePosition(position.id, {
        company: values.company,
        roleTitle: values.roleTitle,
        description: values.description,
        appliedOn: rfcDate,
        url: values.url,
        status: values.status as PositionStatus,
      });

      toast.success("Position updated successfully");
      onSuccess?.();
    } catch (error) {
      UiErrorHandler.handle(error, "Failed to update position");
    }
  };

  return {
    form,
    onSubmit: form.handleSubmit(onSubmit),
    isSubmitting: form.formState.isSubmitting,
  };
}
