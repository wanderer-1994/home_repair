/**
 * @generated SignedSource<<c2831c2cbc498f88bb7962a754bfe4a7>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type AccountType = "CUSTOMER" | "HANDYMAN" | "%future added value";
export type UserSignInWithPasswordInput = {
  accountType: AccountType;
  password: string;
  phoneNumber: string;
};
export type loginWithPasswordMutation$variables = {
  input: UserSignInWithPasswordInput;
};
export type loginWithPasswordMutation$data = {
  readonly userSignInWithPassword: {
    readonly session: {
      readonly actorType: {
        readonly __typename: "Customer";
        readonly id: string;
        readonly phoneNumber: string;
        readonly profile: {
          readonly id: string;
          readonly nickName: string;
        } | null;
      } | {
        // This will never be '%other', but we need some
        // value in case none of the concrete values match.
        readonly __typename: "%other";
      };
      readonly id: string;
    };
  };
};
export type loginWithPasswordMutation = {
  response: loginWithPasswordMutation$data;
  variables: loginWithPasswordMutation$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "defaultValue": null,
    "kind": "LocalArgument",
    "name": "input"
  }
],
v1 = [
  {
    "kind": "Variable",
    "name": "input",
    "variableName": "input"
  }
],
v2 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "id",
  "storageKey": null
},
v3 = {
  "alias": null,
  "args": null,
  "kind": "ScalarField",
  "name": "__typename",
  "storageKey": null
},
v4 = {
  "kind": "InlineFragment",
  "selections": [
    (v2/*: any*/),
    {
      "alias": null,
      "args": null,
      "kind": "ScalarField",
      "name": "phoneNumber",
      "storageKey": null
    },
    {
      "alias": null,
      "args": null,
      "concreteType": "CustomerProfile",
      "kind": "LinkedField",
      "name": "profile",
      "plural": false,
      "selections": [
        (v2/*: any*/),
        {
          "alias": null,
          "args": null,
          "kind": "ScalarField",
          "name": "nickName",
          "storageKey": null
        }
      ],
      "storageKey": null
    }
  ],
  "type": "Customer",
  "abstractKey": null
};
return {
  "fragment": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "loginWithPasswordMutation",
    "selections": [
      {
        "alias": null,
        "args": (v1/*: any*/),
        "concreteType": "UserSignInWithPasswordPayload",
        "kind": "LinkedField",
        "name": "userSignInWithPassword",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "concreteType": "Session",
            "kind": "LinkedField",
            "name": "session",
            "plural": false,
            "selections": [
              (v2/*: any*/),
              {
                "alias": null,
                "args": null,
                "concreteType": null,
                "kind": "LinkedField",
                "name": "actorType",
                "plural": false,
                "selections": [
                  (v3/*: any*/),
                  (v4/*: any*/)
                ],
                "storageKey": null
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ],
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "loginWithPasswordMutation",
    "selections": [
      {
        "alias": null,
        "args": (v1/*: any*/),
        "concreteType": "UserSignInWithPasswordPayload",
        "kind": "LinkedField",
        "name": "userSignInWithPassword",
        "plural": false,
        "selections": [
          {
            "alias": null,
            "args": null,
            "concreteType": "Session",
            "kind": "LinkedField",
            "name": "session",
            "plural": false,
            "selections": [
              (v2/*: any*/),
              {
                "alias": null,
                "args": null,
                "concreteType": null,
                "kind": "LinkedField",
                "name": "actorType",
                "plural": false,
                "selections": [
                  (v3/*: any*/),
                  (v4/*: any*/),
                  {
                    "kind": "InlineFragment",
                    "selections": [
                      (v2/*: any*/)
                    ],
                    "type": "Node",
                    "abstractKey": "__isNode"
                  }
                ],
                "storageKey": null
              }
            ],
            "storageKey": null
          }
        ],
        "storageKey": null
      }
    ]
  },
  "params": {
    "cacheID": "b9adfbad72ccbb938ae0bb71d0b91ce0",
    "id": null,
    "metadata": {},
    "name": "loginWithPasswordMutation",
    "operationKind": "mutation",
    "text": "mutation loginWithPasswordMutation(\n  $input: UserSignInWithPasswordInput!\n) {\n  userSignInWithPassword(input: $input) {\n    session {\n      id\n      actorType {\n        __typename\n        ... on Customer {\n          id\n          phoneNumber\n          profile {\n            id\n            nickName\n          }\n        }\n        ... on Node {\n          __isNode: __typename\n          id\n        }\n      }\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "c78028bc73fd00a327a9f96e670ca07b";

export default node;
