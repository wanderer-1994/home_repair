import { appTestQuery } from "@/__generated__/appTestQuery.graphql";
import Config from "@/config";
import { Text, View } from "react-native";
import { graphql, useLazyLoadQuery } from "react-relay";

export default function Index() {
  const data = useLazyLoadQuery<appTestQuery>(
    graphql`
      query appTestQuery {
        test
      }
    `,
    {},
    {fetchPolicy: "network-only"}
  );

  const names = data.test;

  return (
    <View
      style={{
        flex: 1,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Text className="text-lg font-bold text-red-700">Edit app/index.tsx to edit this screen.</Text>
      <Text className="text-lg font-bold text-red-700">{Config.backend.graphqlPath}</Text>
      <Text className="text-lg font-bold text-red-700">{process.env.EXPO_PUBLIC_APP_ENV}</Text>
      {names.map(name => <Text key={name}>{name}</Text>)}
    </View>
  );
}
