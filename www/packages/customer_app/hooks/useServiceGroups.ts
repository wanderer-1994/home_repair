import { useServiceGroupsQuery } from "@/__generated__/useServiceGroupsQuery.graphql";
import { graphql, useLazyLoadQuery } from "react-relay";

export function useServiceGroups() {
  const data = useLazyLoadQuery<useServiceGroupsQuery>(
    graphql`
      query useServiceGroupsQuery {
        serviceGroups {
          groupType
          children {
            serviceType
          }
        }
      }
    `,
    {},
  );
  return data.serviceGroups;
}
