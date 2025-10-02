import Config from "@/config";
import { authTokenStore } from "@/lib/auth_token_store";
import Constants from "expo-constants";
import {
  Environment,
  FetchFunction,
  Network,
  RecordSource,
  Store,
} from "relay-runtime";

let graphqlEndpoint: string | null = null;

function getGraphqlEndpoint(): string {
  if (graphqlEndpoint !== null) {
    return graphqlEndpoint;
  }
  const apiHost =
    Config.backend.host || Constants.expoConfig?.hostUri?.replace(/:\d*$/, "");
  if (apiHost === undefined) {
    throw Error(
      "Expect either GRAPHQL_ENDPOINT or development host URI must be configured",
    );
  }
  const { https, graphqlPath, port } = Config.backend;
  graphqlEndpoint = `${https ? "https" : "http"}://${apiHost}:${port}/${graphqlPath}`;
  return graphqlEndpoint;
}

const fetchGraphQL: FetchFunction = async (request, variables) => {
  let accessToken = await authTokenStore.getAccessToken();
  const resp = await fetch(getGraphqlEndpoint(), {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Cookie: `ACCESS_TOKEN=${accessToken}`,
    },
    body: JSON.stringify({ query: request.text, variables }),
  });
  if (!resp.ok) {
    throw new Error("Response failed.");
  }
  return await resp.json();
};

const environment = new Environment({
  network: Network.create(fetchGraphQL),
  store: new Store(new RecordSource()),
});

export default environment;
