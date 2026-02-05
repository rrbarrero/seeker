"use client";

import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
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

type FormValues = z.infer<typeof formSchema>;

interface CreatePositionFormProps {
  onSuccess: () => void;
}

export function CreatePositionForm({ onSuccess }: CreatePositionFormProps) {
  const form = useForm<FormValues>({
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

  async function onSubmit(values: FormValues) {
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
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Add New Position</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
            <FormField
              control={form.control}
              name="company"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Company</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g. Acme Corp" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="role_title"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Role Title</FormLabel>
                  <FormControl>
                    <Input placeholder="e.g. Senior Developer" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="applied_on"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Applied On</FormLabel>
                  <FormControl>
                    <Input type="date" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="url"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Job URL</FormLabel>
                  <FormControl>
                    <Input placeholder="https://..." {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="status"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Status</FormLabel>
                  <FormControl>
                    <select
                      className="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
                      {...field}
                    >
                      <option value="CvSent">CV Sent</option>
                      <option value="PhoneScreenScheduled">Phone Screen Scheduled</option>
                      <option value="TechnicalInterview">Technical Interview</option>
                      <option value="OfferReceived">Offer Received</option>
                      <option value="Rejected">Rejected</option>
                      <option value="Withdrawn">Withdrawn</option>
                    </select>
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <textarea
                      className="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[80px] w-full rounded-md border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
                      placeholder="Job description..."
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" className="w-full">
              Create Position
            </Button>
          </form>
        </Form>
      </CardContent>
    </Card>
  );
}
