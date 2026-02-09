import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { toast } from "sonner";
import { UiErrorHandler } from "@/shared/presentation/error-handler";
import { positionService } from "../../composition-root";
import { POSITION_STATUSES, type PositionStatus } from "../../domain/position";
import { PositionUrl } from "../../domain/value-objects/position-url";

const formSchema = z.object({
  company: z.string().min(1, "Company is required"),
  roleTitle: z.string().min(1, "Role title is required"),
  description: z.string().min(1, "Description is required"),
  appliedOn: z.string().min(1, "Date is required"),
  url: z.string().refine(
    (val) => {
      if (val === "") return true;
      try {
        new PositionUrl(val);
        return true;
      } catch {
        return false;
      }
    },
    { message: "Must be a valid URL" },
  ),
  initialComment: z.string(),
  status: z.enum(POSITION_STATUSES),
});

export const createPositionFormSchema = formSchema;

export type CreatePositionFormValues = z.infer<typeof formSchema>;

interface UseCreatePositionFormProps {
  onSuccess: () => void;
}

export function useCreatePositionForm({ onSuccess }: UseCreatePositionFormProps) {
  const form = useForm<CreatePositionFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      company: "",
      roleTitle: "",
      description: "",
      appliedOn: new Date().toISOString().split("T")[0],
      url: "",
      initialComment: "",
      status: "CvSent",
    },
  });

  const onSubmit = async (values: CreatePositionFormValues) => {
    try {
      const rfcDate = new Date(values.appliedOn).toUTCString();

      await positionService.createPosition({
        ...values,
        description: values.description,
        initialComment: values.initialComment,
        url: values.url,
        appliedOn: rfcDate,
        status: values.status as PositionStatus,
      });
      toast.success("Position created successfully");
      form.reset();
      onSuccess();
    } catch (error) {
      UiErrorHandler.handle(error, "Failed to create position");
    }
  };

  return {
    form,
    onSubmit: form.handleSubmit(onSubmit),
    isSubmitting: form.formState.isSubmitting,
  };
}
