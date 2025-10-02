import { Tabs } from "expo-router";

export const unstable_settings = {
  anchor: "(tabs)",
};

export default function Layout() {
  return (
    <Tabs initialRouteName="home">
      <Tabs.Screen
        name="blog"
        options={{
          title: "Blog",
          headerShown: false,
        }}
      />
      <Tabs.Screen
        name="my_activity"
        options={{
          title: "Ativity",
          headerShown: false,
        }}
      />
      <Tabs.Screen
        name="home"
        options={{
          title: "Home",
          headerShown: false,
        }}
      />
      <Tabs.Screen
        name="find_handyman"
        options={{
          title: "Thợ",
          headerShown: false,
        }}
      />
      <Tabs.Screen
        name="message"
        options={{
          title: "Message",
          headerShown: false,
        }}
      />
    </Tabs>
  );
}
