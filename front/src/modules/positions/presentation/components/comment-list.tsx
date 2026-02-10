import { useState } from "react";
import { MessageSquare, Pencil, Trash2, X } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { UiErrorHandler } from "@/shared/presentation/error-handler";
import { commentService } from "../../composition-root";
import type { CommentProps } from "../../domain/comment";

interface CommentListProps {
  comments: CommentProps[];
  positionId: string;
  currentUserId: string | null;
  onUpdated: (comment: CommentProps) => void;
  onDeleted: (commentId: string) => void;
}

export function CommentList({
  comments,
  positionId,
  currentUserId,
  onUpdated,
  onDeleted,
}: CommentListProps) {
  const [editingId, setEditingId] = useState<string | null>(null);
  const [draftBody, setDraftBody] = useState("");
  const [busyId, setBusyId] = useState<string | null>(null);

  if (comments.length === 0) {
    return (
      <div className="text-muted-foreground flex items-center gap-2 text-sm">
        <MessageSquare className="h-4 w-4" />
        <span>No comments yet.</span>
      </div>
    );
  }

  const handleEdit = (comment: CommentProps) => {
    setEditingId(comment.id);
    setDraftBody(comment.body);
  };

  const handleCancel = () => {
    setEditingId(null);
    setDraftBody("");
  };

  const handleSave = async (commentId: string) => {
    if (!draftBody.trim()) {
      toast.error("Comment cannot be empty");
      return;
    }

    setBusyId(commentId);
    try {
      const updated = await commentService.updateComment(positionId, commentId, {
        body: draftBody,
      });
      onUpdated(updated.toPrimitives());
      setEditingId(null);
      toast.success("Comment updated");
    } catch (error) {
      UiErrorHandler.handle(error, "Failed to update comment");
    } finally {
      setBusyId(null);
    }
  };

  const handleDelete = async (commentId: string) => {
    setBusyId(commentId);
    try {
      await commentService.deleteComment(positionId, commentId);
      onDeleted(commentId);
      toast.success("Comment deleted");
    } catch (error) {
      UiErrorHandler.handle(error, "Failed to delete comment");
    } finally {
      setBusyId(null);
    }
  };

  return (
    <div className="space-y-3">
      {comments.map((comment) => {
        const isOwner = currentUserId && comment.userId === currentUserId;
        const isEditing = editingId === comment.id;
        const isBusy = busyId === comment.id;

        return (
          <div
            key={comment.id}
            className="rounded-lg border bg-zinc-50 p-4 text-sm text-zinc-700 shadow-sm dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-200"
          >
            <div className="flex items-start justify-between gap-3">
              <div className="flex-1">
                {isEditing ? (
                  <textarea
                    className="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex min-h-[80px] w-full rounded-md border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none"
                    value={draftBody}
                    onChange={(event) => setDraftBody(event.target.value)}
                    disabled={isBusy}
                  />
                ) : (
                  <p className="leading-relaxed whitespace-pre-wrap">{comment.body}</p>
                )}

                <p className="text-muted-foreground mt-2 text-[11px]">
                  {new Date(comment.createdAt).toLocaleString()}
                </p>
              </div>

              {isOwner && (
                <div className="flex items-center gap-2">
                  {isEditing ? (
                    <>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleSave(comment.id)}
                        disabled={isBusy}
                      >
                        Save
                      </Button>
                      <Button variant="ghost" size="sm" onClick={handleCancel} disabled={isBusy}>
                        <X className="h-4 w-4" />
                      </Button>
                    </>
                  ) : (
                    <>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleEdit(comment)}
                        disabled={isBusy}
                      >
                        <Pencil className="h-4 w-4" />
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleDelete(comment.id)}
                        disabled={isBusy}
                      >
                        <Trash2 className="h-4 w-4 text-red-500" />
                      </Button>
                    </>
                  )}
                </div>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}
