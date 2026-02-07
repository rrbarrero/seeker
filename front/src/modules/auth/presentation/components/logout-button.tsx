"use client";

import { useRouter } from "next/navigation";
import { toast } from "sonner";
import { LogOut } from "lucide-react";

import { Button } from "@/components/ui/button";
import { authService } from "../../composition-root";

export function LogoutButton() {
  const router = useRouter();

  const handleLogout = () => {
    authService.logout();
    toast.success("Logged out successfully");
    router.push("/auth/login");
  };

  return (
    <Button variant="ghost" size="sm" onClick={handleLogout} className="flex gap-2">
      <LogOut className="h-4 w-4" />
      Logout
    </Button>
  );
}
