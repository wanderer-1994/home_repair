import useSessionCheck from "@/hooks/useSessionCheck";
import { useRouter } from "expo-router";
import React from "react";
import { ActivityIndicator } from "react-native";

export default function Index() {
  const router = useRouter();
  const check = useSessionCheck();

  React.useEffect(() => {
    if (check._status === "UNAUTHENTICATED") {
      router.replace("/(auth)");
    } else {
      router.replace("/(tabs)");
    }
  }, [check._status, router]);

  return <ActivityIndicator />;
}
