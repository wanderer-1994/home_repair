import { useSessionCheck } from "@/hooks/useSessionCheck";
import { useRouter } from "expo-router";
import React from "react";

/**
 * Checks for an active user session.
 * If found, navigates user to dashboard screen.
 */
export function useRedirectIfAuthenticated() {
  const router = useRouter();
  const check = useSessionCheck();

  React.useCallback(() => {
    if (check._status === "AUTHENTICATED") {
      router.replace("/(tabs)/home");
    }
  }, [check._status, router]);
}
