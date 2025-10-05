import { useServiceGroupsQuery } from "@/__generated__/useServiceGroupsQuery.graphql";
import { graphql, useLazyLoadQuery } from "react-relay";

export function useServiceGroups() {
  const data = useLazyLoadQuery<useServiceGroupsQuery>(
    graphql`
      query useServiceGroupsQuery {
        serviceGroups {
          groupType
          children {
            __typename
            ... on ServiceAirConditionerFixing {
              foo
            }
            ... on ServiceAirConditionerCleaning {
              foo
            }
            ... on ServiceWashingMachineFixing {
              foo
            }
            ... on ServiceWashingMachineCleaning {
              foo
            }
            ... on ServiceOther {
              foo
            }
          }
        }
      }
    `,
    {},
  );
  return data.serviceGroups;
}
