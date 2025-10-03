/**
 * @generated SignedSource<<7ec648dd296b733357e6c32e7fd12a5e>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type AccountType = "CUSTOMER" | "HANDYMAN" | "%future added value";
export type UserAccountStartRegistrationInput = {
  accountType: AccountType;
  phoneNumber: string;
};
export type signupStartRegistrationMutation$variables = {
  input: UserAccountStartRegistrationInput;
};
export type signupStartRegistrationMutation$data = {
  readonly userAccountStartRegistration: {
    readonly case: {
      readonly __typename: "StartRegistrationCaseAccountExist";
      readonly foo: boolean;
    } | {
      readonly __typename: "StartRegistrationCaseOtpCode";
      readonly digits: number;
      readonly e164PhoneNumberStr: string;
      readonly ttlSeconds: number;
    } | {
      // This will never be '%other', but we need some
      // value in case none of the concrete values match.
      readonly __typename: "%other";
    };
  };
};
export type signupStartRegistrationMutation = {
  response: signupStartRegistrationMutation$data;
  variables: signupStartRegistrationMutation$variables;
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
    "alias": null,
    "args": [
      {
        "kind": "Variable",
        "name": "input",
        "variableName": "input"
      }
    ],
    "concreteType": "UserAccountStartRegistrationPayload",
    "kind": "LinkedField",
    "name": "userAccountStartRegistration",
    "plural": false,
    "selections": [
      {
        "alias": null,
        "args": null,
        "concreteType": null,
        "kind": "LinkedField",
        "name": "case",
        "plural": false,
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
            "selections": [
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "foo",
                "storageKey": null
              }
            ],
            "type": "StartRegistrationCaseAccountExist",
            "abstractKey": null
          },
          {
            "kind": "InlineFragment",
            "selections": [
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "digits",
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "ttlSeconds",
                "storageKey": null
              },
              {
                "alias": null,
                "args": null,
                "kind": "ScalarField",
                "name": "e164PhoneNumberStr",
                "storageKey": null
              }
            ],
            "type": "StartRegistrationCaseOtpCode",
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
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "signupStartRegistrationMutation",
    "selections": (v1/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "signupStartRegistrationMutation",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "b9d5cc3faa1400109a1a42233dc920d1",
    "id": null,
    "metadata": {},
    "name": "signupStartRegistrationMutation",
    "operationKind": "mutation",
    "text": "mutation signupStartRegistrationMutation(\n  $input: UserAccountStartRegistrationInput!\n) {\n  userAccountStartRegistration(input: $input) {\n    case {\n      __typename\n      ... on StartRegistrationCaseAccountExist {\n        foo\n      }\n      ... on StartRegistrationCaseOtpCode {\n        digits\n        ttlSeconds\n        e164PhoneNumberStr\n      }\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "30ddcfaaa0f22625568d87491cf90a35";

export default node;
