"use client";

import { useEffect, useState } from "react";
import { authService } from "@/modules/auth/composition-root";
import { AlertCircle } from "lucide-react";

export function EmailVerificationBanner() {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    // Check local storage safely
    const checkStatus = () => {
      const token = authService.getToken();
      const isValidated = authService.isEmailValidated();

      // Only show if user has a token (is logged in) and email is NOT validated
      if (token && !isValidated) {
        setIsVisible(true);
      } else {
        setIsVisible(false);
      }
    };

    checkStatus();
    // Optional: listen to storage events if we want cross-tab sync,
    // but for now simple mount check is enough.
  }, []);

  if (!isVisible) return null;

  return (
    <div className="mb-6 rounded-md border border-yellow-200 bg-yellow-50 p-4 dark:border-yellow-900/50 dark:bg-yellow-900/20">
      <div className="flex items-start gap-3">
        <AlertCircle className="mt-0.5 h-5 w-5 text-yellow-600 dark:text-yellow-500" />
        <div className="flex-1">
          <h3 className="text-sm font-medium text-yellow-800 dark:text-yellow-400">
            Email not verified
          </h3>
          <p className="mt-1 text-sm text-yellow-700 dark:text-yellow-500/90">
            Please check your email inbox and click the verification link to fully activate your
            account.
          </p>
        </div>
      </div>
    </div>
  );
}
