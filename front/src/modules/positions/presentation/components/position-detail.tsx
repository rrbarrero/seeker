"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import {
  ArrowLeft,
  ExternalLink,
  Calendar,
  Building2,
  Briefcase,
  MessageSquare,
  Trash2,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { Position, type PositionProps } from "../../domain/position";
import { useDeletePosition } from "../hooks/use-delete-position";

interface PositionDetailProps {
  position: PositionProps;
}

export function PositionDetail({ position: props }: PositionDetailProps) {
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const router = useRouter();
  const { deletePosition, isDeleting } = useDeletePosition();
  const position = Position.fromPrimitives(props);

  const handleDelete = async () => {
    await deletePosition(position.id);
    setShowDeleteDialog(false);
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <Button
          variant="ghost"
          onClick={() => router.back()}
          className="flex items-center gap-2 hover:bg-zinc-100 dark:hover:bg-zinc-800"
        >
          <ArrowLeft className="h-4 w-4" />
          Back to Dashboard
        </Button>
      </div>

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        {/* Main Details */}
        <div className="space-y-6 lg:col-span-2">
          <Card className="overflow-hidden border-none shadow-sm dark:bg-zinc-900">
            <div className="bg-primary h-2" />
            <CardHeader className="pb-4">
              <div className="flex items-start justify-between">
                <div className="space-y-1">
                  <CardTitle className="text-3xl font-bold tracking-tight">
                    {position.role_title}
                  </CardTitle>
                  <div className="text-muted-foreground flex items-center gap-2 text-xl">
                    <Building2 className="h-5 w-5" />
                    <span>{position.company}</span>
                  </div>
                </div>
                <div className="bg-primary/10 text-primary rounded-full px-3 py-1 text-sm font-semibold">
                  {position.status}
                </div>
              </div>
            </CardHeader>
            <CardContent className="space-y-6">
              <div className="space-y-2">
                <h3 className="flex items-center gap-2 text-lg font-semibold">
                  <Briefcase className="text-primary h-5 w-5" />
                  Description
                </h3>
                <div className="rounded-lg border bg-zinc-50 p-4 dark:border-zinc-800 dark:bg-zinc-800/50">
                  <p className="leading-relaxed whitespace-pre-wrap text-zinc-600 dark:text-zinc-400">
                    {position.description || "No description provided."}
                  </p>
                </div>
              </div>

              {position.initial_comment && (
                <div className="space-y-2">
                  <h3 className="flex items-center gap-2 text-lg font-semibold">
                    <MessageSquare className="text-primary h-5 w-5" />
                    Initial Comments
                  </h3>
                  <div className="bg-primary/5 border-primary/10 rounded-lg border p-4 text-zinc-600 italic dark:text-zinc-400">
                    &ldquo;{position.initial_comment}&rdquo;
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        </div>

        {/* Sidebar Info */}
        <div className="space-y-6">
          <Card className="border-none shadow-sm dark:bg-zinc-900">
            <CardHeader>
              <CardTitle className="text-lg font-bold">Metadata</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-start gap-3">
                <Calendar className="text-muted-foreground mt-0.5 h-5 w-5 shrink-0" />
                <div className="space-y-0.5">
                  <p className="text-sm font-medium">Applied On</p>
                  <p className="text-muted-foreground text-xs">
                    {position.getFormattedAppliedDate()}
                  </p>
                </div>
              </div>

              {position.url && (
                <div className="flex items-start gap-3">
                  <ExternalLink className="text-muted-foreground mt-0.5 h-5 w-5 shrink-0" />
                  <div className="space-y-0.5 overflow-hidden">
                    <p className="text-sm font-medium">Job URL</p>
                    <a
                      href={position.url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-primary block truncate text-xs hover:underline"
                    >
                      {position.url}
                    </a>
                  </div>
                </div>
              )}

              <div className="border-t pt-4 dark:border-zinc-800">
                <p className="text-muted-foreground mb-2 text-[10px] font-bold tracking-wider uppercase">
                  Record Created
                </p>
                <p className="text-muted-foreground text-xs">
                  {new Date(position.created_at).toLocaleString()}
                </p>
              </div>
            </CardContent>
          </Card>

          <div className="flex flex-col gap-3">
            <Button className="h-12 w-full text-lg font-semibold" disabled>
              Edit (Coming Soon)
            </Button>
            <Button
              variant="destructive"
              className="h-12 w-full text-lg font-semibold"
              onClick={() => setShowDeleteDialog(true)}
              disabled={isDeleting}
            >
              <Trash2 className="mr-2 h-5 w-5" />
              {isDeleting ? "Deleting..." : "Delete Position"}
            </Button>
          </div>
        </div>
      </div>

      <AlertDialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the position for{" "}
              <span className="font-semibold text-zinc-900 dark:text-zinc-100">
                {position.role_title}
              </span>{" "}
              at{" "}
              <span className="font-semibold text-zinc-900 dark:text-zinc-100">
                {position.company}
              </span>
              .
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDelete}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
              disabled={isDeleting}
            >
              {isDeleting ? "Deleting..." : "Delete Position"}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}
