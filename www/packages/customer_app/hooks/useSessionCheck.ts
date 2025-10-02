import { useSessionCheckQuery } from "@/__generated__/useSessionCheckQuery.graphql";
import { newClientError } from "@/lib/client_error";
import { graphql, useLazyLoadQuery } from "react-relay";

/**
 * Check if a session exist.
 */
export default function useSessionCheck() {
  const session = useLazyLoadQuery<useSessionCheckQuery>(
    graphql`
      query useSessionCheckQuery {
        session {
          id
          iat
          exp
          actorType {
            __typename
            ... on Customer {
              id
              phoneNumber
              profile {
                id
                nickName
              }
            }
          }
        }
      }
    `,
    {},
    { fetchPolicy: "network-only" },
  ).session;

  if (session === null) {
    return {
      _status: "UNAUTHENTICATED" as const,
      session: null,
    };
  }

  const sessionType = session.actorType.__typename;
  if (sessionType !== "Customer") {
    throw newClientError("Unexpected non-customer session");
  }

  return {
    _status: "AUTHENTICATED" as const,
    session,
    customer: session.actorType,
  };
}
