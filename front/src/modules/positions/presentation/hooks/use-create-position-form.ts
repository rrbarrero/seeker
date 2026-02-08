import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { toast } from "sonner";
import { positionService } from "../../composition-root";
import type { PositionStatus } from "../../domain/position";

const formSchema = z.object({
  company: z.string().min(1, "Company is required"),
  role_title: z.string().min(1, "Role title is required"),
  description: z.string().min(1, "Description is required"),
  applied_on: z.string().min(1, "Date is required"),
  url: z.string().refine((val) => val === "" || /^https?:\/\/.+/.test(val), {
    message: "Must be a valid URL",
  }),
  initial_comment: z.string(),
  status: z.enum([
    "CvSent",
    "PhoneScreenScheduled",
    "TechnicalInterview",
    "OfferReceived",
    "Rejected",
    "Withdrawn",
  ] as [string, ...string[]]),
});

export type CreatePositionFormValues = z.infer<typeof formSchema>;

interface UseCreatePositionFormProps {
  onSuccess: () => void;
}

export function useCreatePositionForm({ onSuccess }: UseCreatePositionFormProps) {
  const form = useForm<CreatePositionFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      company: "",
      role_title: "",
      description: "",
      applied_on: new Date().toISOString().split("T")[0],
      url: "",
      initial_comment: "",
      status: "CvSent",
    },
  });

  const onSubmit = async (values: CreatePositionFormValues) => {
    try {
      const rfcDate = new Date(values.applied_on).toUTCString();

      await positionService.createPosition({
        ...values,
        description: values.description,
        initial_comment: values.initial_comment,
        url: values.url,
        applied_on: rfcDate,
        status: values.status as PositionStatus,
      });
      toast.success("Position created successfully");
      form.reset();
      onSuccess();
    } catch (error) {
      console.error(error);
      toast.error("Failed to create position");
    }
  };

  return {
    form,
    onSubmit: form.handleSubmit(onSubmit),
    isSubmitting: form.formState.isSubmitting,
  };
}
