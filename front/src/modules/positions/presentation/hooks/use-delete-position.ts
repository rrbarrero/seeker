import { useState } from "react";
import { useRouter } from "next/navigation";
import { toast } from "sonner";
import { UiErrorHandler } from "@/shared/presentation/error-handler";
import { positionService } from "../../composition-root";

interface UseDeletePositionProps {
  onSuccess?: () => void;
}

export function useDeletePosition({ onSuccess }: UseDeletePositionProps = {}) {
  const [isDeleting, setIsDeleting] = useState(false);
  const router = useRouter();

  const deletePosition = async (id: string) => {
    setIsDeleting(true);
    try {
      await positionService.deletePosition(id);
      toast.success("Position deleted successfully");
      onSuccess?.();
      router.push("/dashboard");
      router.refresh();
    } catch (error) {
      UiErrorHandler.handle(error, "Failed to delete position");
    } finally {
      setIsDeleting(false);
    }
  };

  return {
    deletePosition,
    isDeleting,
  };
}
