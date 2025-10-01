/**
 * @generated SignedSource<<e1e5061f9930c4e9df34bb46bb414d20>>
 * @lightSyntaxTransform
 * @nogrep
 */

/* tslint:disable */
/* eslint-disable */
// @ts-nocheck

import { ConcreteRequest } from 'relay-runtime';
export type appTestQuery$variables = Record<PropertyKey, never>;
export type appTestQuery$data = {
  readonly test: ReadonlyArray<string>;
};
export type appTestQuery = {
  response: appTestQuery$data;
  variables: appTestQuery$variables;
};

const node: ConcreteRequest = (function(){
var v0 = [
  {
    "alias": null,
    "args": null,
    "kind": "ScalarField",
    "name": "test",
    "storageKey": null
  }
];
return {
  "fragment": {
    "argumentDefinitions": [],
    "kind": "Fragment",
    "metadata": null,
    "name": "appTestQuery",
    "selections": (v0/*: any*/),
    "type": "Query",
    "abstractKey": null
  },
  "kind": "Request",
  "operation": {
    "argumentDefinitions": [],
    "kind": "Operation",
    "name": "appTestQuery",
    "selections": (v0/*: any*/)
  },
  "params": {
    "cacheID": "8d021cf2af36003f93367d1b1134055b",
    "id": null,
    "metadata": {},
    "name": "appTestQuery",
    "operationKind": "query",
    "text": "query appTestQuery {\n  test\n}\n"
  }
};
})();

(node as any).hash = "c33d89a1086664048896bfc6a013a702";

export default node;
