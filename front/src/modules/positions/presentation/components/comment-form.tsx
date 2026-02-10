"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";
import { UiErrorHandler } from "@/shared/presentation/error-handler";
import type { CommentProps } from "../../domain/comment";
import { commentService } from "../../composition-root";

interface CommentFormProps {
  positionId: string;
  onCreated: (comment: CommentProps) => void;
}

export function CommentForm({ positionId, onCreated }: CommentFormProps) {
  const [body, setBody] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!body.trim()) {
      toast.error("Comment cannot be empty");
      return;
    }

    setIsSubmitting(true);
    try {
      const created = await commentService.createComment(positionId, { body });
      onCreated(created.toPrimitives());
      setBody("");
      toast.success("Comment added");
    } catch (error) {
      UiErrorHandler.handle(error, "Failed to add comment");
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <textarea
        className="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[100px] w-full rounded-md border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50"
        placeholder="Add a note about the interview..."
        value={body}
        onChange={(event) => setBody(event.target.value)}
        disabled={isSubmitting}
      />
      <div className="flex justify-end">
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting ? "Saving..." : "Add Comment"}
        </Button>
      </div>
    </form>
  );
}
