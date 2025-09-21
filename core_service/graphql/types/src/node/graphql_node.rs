use crate::*;
use async_graphql::{ID, Interface};
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// The key that identifies a node type. This is used to parse the global id.
/// For new node types, append a new variant to this enum.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Encode, Decode)]
pub enum NodeKey {
    FooNode = 1,
    BarNode,
    Account,
    Session,
}

/// Identifies a global object uniquely.
/// See <https://graphql.org/learn/global-object-identification/>
#[derive(Interface)]
#[graphql(field(name = "id", ty = "ID"))]
pub enum Node {
    FooNode(FooNode),
    BarNode(BarNode),
    Account(Account),
    Session(Session),
}
