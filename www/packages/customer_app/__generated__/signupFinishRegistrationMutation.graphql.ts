/**
 * @generated SignedSource<<2fda766865bded5f71f1425a9c0ce78d>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type AccountType = "CUSTOMER" | "HANDYMAN" | "%future added value";
export type UserAccountFinishRegistrationInput = {
  accountType: AccountType;
  otpCode: string;
  password: string;
  phoneNumber: string;
};
export type signupFinishRegistrationMutation$variables = {
  input: UserAccountFinishRegistrationInput;
};
export type signupFinishRegistrationMutation$data = {
  readonly userAccountFinishRegistration: {
    readonly session: {
      readonly id: string;
    };
  };
};
export type signupFinishRegistrationMutation = {
  response: signupFinishRegistrationMutation$data;
  variables: signupFinishRegistrationMutation$variables;
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
    "concreteType": "UserAccountFinishRegistrationPayload",
    "kind": "LinkedField",
    "name": "userAccountFinishRegistration",
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
          {
            "alias": null,
            "args": null,
            "kind": "ScalarField",
            "name": "id",
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
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Fragment",
    "metadata": null,
    "name": "signupFinishRegistrationMutation",
    "selections": (v1/*: any*/),
    "type": "Mutation",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": (v0/*: any*/),
    "kind": "Operation",
    "name": "signupFinishRegistrationMutation",
    "selections": (v1/*: any*/)
  },
  "params": {
    "cacheID": "2b2f0c2b8672908616d7bb15bd2f8653",
    "id": null,
    "metadata": {},
    "name": "signupFinishRegistrationMutation",
    "operationKind": "mutation",
    "text": "mutation signupFinishRegistrationMutation(\n  $input: UserAccountFinishRegistrationInput!\n) {\n  userAccountFinishRegistration(input: $input) {\n    session {\n      id\n    }\n  }\n}\n"
  }
};
})();

(node as any).hash = "fbe7545d68e5b5dbe176c6fe962de1e4";

export default node;
