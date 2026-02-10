"use client";

import { useCallback, useEffect, useState } from "react";
import { useRouter } from "next/navigation";

import { PositionList } from "@/modules/positions/presentation/components/position-list";
import { CreatePositionForm } from "@/modules/positions/presentation/components/create-position-form";
import type { PositionProps } from "@/modules/positions/domain/position";
import { positionService } from "@/modules/positions/composition-root";
import { LogoutButton } from "@/modules/auth/presentation/components/logout-button";
import { authService } from "@/modules/auth/composition-root";
import { UiErrorHandler } from "@/shared/presentation/error-handler";
import { UnauthorizedError } from "@/shared/domain/errors";
import { EmailVerificationBanner } from "@/modules/auth/presentation/components/email-verification-banner";

export default function DashboardPage() {
  const [positions, setPositions] = useState<PositionProps[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();

  const fetchPositions = useCallback(async () => {
    try {
      const data = await positionService.getPositions();
      const primitives = data.filter((p) => !p.deleted).map((p) => p.toPrimitives());
      const sortedPositions = [...primitives].sort(
        (a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime(),
      );
      setPositions(sortedPositions);
    } catch (error) {
      UiErrorHandler.handle(error, "Error loading positions");
      if (error instanceof UnauthorizedError) {
        authService.logout();
        router.push("/auth/login");
      }
    } finally {
      setIsLoading(false);
    }
  }, [router]);

  useEffect(() => {
    fetchPositions();
  }, [fetchPositions]);

  const handlePositionCreated = () => {
    fetchPositions();
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <EmailVerificationBanner />
      <div className="mb-8 flex items-center justify-between">
        <h1 className="text-3xl font-bold tracking-tight">My Applications</h1>
        <LogoutButton />
      </div>

      <div className="flex flex-col gap-8 lg:flex-row">
        {/* Main Content - List */}
        <div className="order-2 flex-1 lg:order-1">
          {isLoading ? (
            <div className="p-8 text-center">Loading positions...</div>
          ) : (
            <PositionList positions={positions} />
          )}
        </div>

        {/* Sidebar - Form */}
        <div className="order-1 w-full lg:order-2 lg:w-[400px]">
          <div className="sticky top-8">
            <CreatePositionForm onSuccess={handlePositionCreated} />
          </div>
        </div>
      </div>
    </div>
  );
}
