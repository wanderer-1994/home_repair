import { useServiceGroupsQuery$data } from "@/__generated__/useServiceGroupsQuery.graphql";
import { useServiceGroups } from "@/hooks/useServiceGroups";
import { toastErr } from "@/lib/toast";
import { Link } from "expo-router";
import React, { Suspense } from "react";
import { ActivityIndicator, FlatList, View } from "react-native";

/**
 * Home/Dashboard Tab: The primary and default landing screen for the customer.
 * ## Purpose:
 * 1. **Feature Discovery:** Highlight the app's core value propositions and best features to encourage adoption.
 * 2. **Navigation Hub:** Provide expressive UI components (e.g., cards, quick links) to quickly navigate the user to commonly used features.
 * 3. **Engagement:** Serve as an overall engagement dashboard to summarize recent activity or personalized information.
 */
export default function Home() {
  return (
    <View>
      <Carousel />
      <Suspense fallback={<ActivityIndicator />}>
        <Services />
      </Suspense>
    </View>
  );
}

/**
 * Carousel highlight 3 core features
 */
function Carousel() {
  return (
    <div className="flex gap-1 items-center">
      <div>
        <p>Blog for tips & tricks</p>
        <Link href="/(tabs)/blog">Blog</Link>
      </div>

      <div>
        <p>Explore high skilled handyman</p>
        <Link href="/(tabs)/find_handyman">Blog</Link>
      </div>

      <div>
        <p>Get you house fix instantly with high quality</p>
      </div>
    </div>
  );
}

/**
 * List all services, when user click to any service, navigate to create job page
 */
function Services() {
  const services = useServiceGroups();

  return (
    <div className="flex gap-1 items-center">
      <FlatList
        data={services}
        renderItem={ServiceItem}
        keyExtractor={(item) => item.groupType}
      />
    </div>
  );
}

type ServiceItemProps = {
  item: useServiceGroupsQuery$data["serviceGroups"][number];
};

function ServiceItem({ item }: ServiceItemProps) {
  switch (item.groupType) {
    case "AIR_CONDITIONER": {
      return <div>AC</div>;
    }
    case "WASHING_MACHINE": {
      return <div>Washing Machine</div>;
    }
    case "OTHER": {
      return <div>Other</div>;
    }
    default: {
      toastErr("Unknown service");
      return null;
    }
  }
}
