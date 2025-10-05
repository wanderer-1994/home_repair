import { Tabs } from "expo-router";

export const unstable_settings = {
  anchor: "(tabs)",
};

/**
 * TODO: Value-Added Features encouraging user interaction with the app
 *  1. Smart Maintenance Calendar remind customer of cleaning AC, washing machine, replace water filter core
 *  2. Chợ đồ cũ
 *  4. Q&A Forum (Hỏi đáp cộng đồng): người dùng đặt câu hỏi. Người dùng khác hoặc thợ có thể trả lời.
 *     Nếu câu trả lời của thợ được like thì thợ sẽ được tăng điểm cộng đồng.
 */
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
