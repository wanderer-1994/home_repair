/**
 * @generated SignedSource<<18304fed0d0545afdeea651583840b69>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type ServiceLayer1 = "AIR_CONDITIONER" | "OTHER" | "WASHING_MACHINE" | "%future added value";
export type ServiceLayer2 = "AIR_CONDITIONER_CLEANING" | "AIR_CONDITIONER_FIXING" | "OTHER" | "WASHING_MACHINE_CLEANING" | "WASHING_MACHINE_FIXING" | "%future added value";
export type useServiceGroupsQuery$variables = Record<PropertyKey, never>;
export type useServiceGroupsQuery$data = {
  readonly serviceGroups: ReadonlyArray<{
    readonly children: ReadonlyArray<{
      readonly serviceType: ServiceLayer2;
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
        "concreteType": "Service",
        "kind": "LinkedField",
        "name": "children",
        "plural": true,
        "selections": [
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "serviceType",
            "storageKey": null
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
    "selections": (v0/*: any*/),
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "useServiceGroupsQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "26449e16fc5626b0b6bcd5abc396d61f",
    "id": null,
    "metadata": {},
    "name": "useServiceGroupsQuery",
    "operationKind": "query",
    "text": "query useServiceGroupsQuery {\n  serviceGroups {\n    groupType\n    children {\n      serviceType\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "f1fb8a68d5c6e583c72d0f163b6db28c";

export default node;
