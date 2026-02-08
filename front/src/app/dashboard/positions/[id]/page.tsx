import { cookies } from "next/headers";
import { notFound } from "next/navigation";
import { positionService } from "@/modules/positions/composition-root";
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
  try {
    position = await positionService.getPosition(id, token);
  } catch (error) {
    console.error("Error fetching position:", error);
    notFound();
  }

  if (!position) {
    notFound();
  }

  return (
    <div className="container mx-auto max-w-5xl px-4 py-8">
      <PositionDetail position={position.toPrimitives()} />
    </div>
  );
}
