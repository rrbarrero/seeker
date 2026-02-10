import { cookies } from "next/headers";
import { notFound } from "next/navigation";
import { NotFoundError } from "@/shared/domain/errors";
import { commentService, positionService } from "@/modules/positions/composition-root";
import { PositionDetail } from "@/modules/positions/presentation/components/position-detail";

interface PageProps {
  params: Promise<{
    id: string;
  }>;
}

export default async function PositionPage({ params }: PageProps) {
  const { id } = await params;
  const cookieStore = await cookies();
  const token = cookieStore.get("token")?.value;

  let position;
  let comments = [];
  try {
    position = await positionService.getPosition(id, token);
    comments = await commentService.getComments(id, token);
  } catch (error) {
    if (error instanceof NotFoundError) {
      notFound();
    }
    throw error;
  }

  if (!position || position.deleted) {
    notFound();
  }

  return (
    <div className="container mx-auto max-w-5xl px-4 py-8">
      <PositionDetail
        position={position.toPrimitives()}
        comments={comments.map((c) => c.toPrimitives())}
      />
    </div>
  );
}
