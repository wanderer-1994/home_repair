/**
 * @generated SignedSource<<82d3389b5c26bc2c9c1631db3e8bbb19>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type ServiceLayer1 = "AIR_CONDITIONER" | "OTHER" | "WASHING_MACHINE" | "%future added value";
export type useServiceGroupsQuery$variables = Record<PropertyKey, never>;
export type useServiceGroupsQuery$data = {
  readonly serviceGroups: ReadonlyArray<{
    readonly children: ReadonlyArray<{
      readonly __typename: "ServiceAirConditionerCleaning";
      readonly foo: boolean;
    } | {
      readonly __typename: "ServiceAirConditionerFixing";
      readonly foo: boolean;
    } | {
      readonly __typename: "ServiceOther";
      readonly foo: boolean;
    } | {
      readonly __typename: "ServiceWashingMachineCleaning";
      readonly foo: boolean;
    } | {
      readonly __typename: "ServiceWashingMachineFixing";
      readonly foo: boolean;
    } | {
      // This will never be '%other', but we need some
      // value in case none of the concrete values match.
      readonly __typename: "%other";
    }>;
    readonly groupType: ServiceLayer1;
  }>;
};
export type useServiceGroupsQuery = {
  response: useServiceGroupsQuery$data;
  variables: useServiceGroupsQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "kind": "ScalarField",
    "name": "foo",
    "storageKey": null
  }
],
v1 = [
  {
    "alias": null,
    "args": null,
    "concreteType": "ServiceGroup",
    "kind": "LinkedField",
    "name": "serviceGroups",
    "plural": true,
    "selections": [
      {
        "alias": null,
        "args": null,
        "kind": "ScalarField",
        "name": "groupType",
        "storageKey": null
      },
      {
        "alias": null,
        "args": null,
        "concreteType": null,
        "kind": "LinkedField",
        "name": "children",
        "plural": true,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "__typename",
            "storageKey": null
          },
          {
            "kind": "InlineFragment",
            "selections": (v0/*: any*/),
            "type": "ServiceAirConditionerFixing",
            "abstractKey": null
          },
          {
            "kind": "InlineFragment",
            "selections": (v0/*: any*/),
            "type": "ServiceAirConditionerCleaning",
            "abstractKey": null
          },
          {
            "kind": "InlineFragment",
            "selections": (v0/*: any*/),
            "type": "ServiceWashingMachineFixing",
            "abstractKey": null
          },
          {
            "kind": "InlineFragment",
            "selections": (v0/*: any*/),
            "type": "ServiceWashingMachineCleaning",
            "abstractKey": null
          },
          {
            "kind": "InlineFragment",
            "selections": (v0/*: any*/),
            "type": "ServiceOther",
            "abstractKey": null
          }
        ],
        "storageKey": null
      }
    ],
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "useServiceGroupsQuery",
    "selections": (v1/*: any*/),
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "useServiceGroupsQuery",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "8bf892109da23d89ee8da0e2e7a33eb2",
    "id": null,
    "metadata": {},
    "name": "useServiceGroupsQuery",
    "operationKind": "query",
    "text": "query useServiceGroupsQuery {\n  serviceGroups {\n    groupType\n    children {\n      __typename\n      ... on ServiceAirConditionerFixing {\n        foo\n      }\n      ... on ServiceAirConditionerCleaning {\n        foo\n      }\n      ... on ServiceWashingMachineFixing {\n        foo\n      }\n      ... on ServiceWashingMachineCleaning {\n        foo\n      }\n      ... on ServiceOther {\n        foo\n      }\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "f0f369b5e03d47bc590a808efd015b28";

export default node;
