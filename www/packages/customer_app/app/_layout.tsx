import environment from "@/lib/relay_environment";
import { Stack } from "expo-router";
import { Suspense } from "react";
import { ActivityIndicator } from "react-native";
import { RelayEnvironmentProvider } from "react-relay";
import "../global.css";

export default function RootLayout() {
  return (
    <RelayEnvironmentProvider environment={environment}>
      <Suspense fallback={<ActivityIndicator />}>
        <Stack />
      </Suspense>
    </RelayEnvironmentProvider>
  );
}
