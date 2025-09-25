use crate::*;
use async_graphql::ID;
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use bincode::{Decode, Encode, config};
use error::{Error, Result};
use serde::{Serialize, de::DeserializeOwned};

/// N/B: we can't use [`Decode`], [`Encode`] traits directly because
/// [`CachedNode`] implemented with [`OnceCell`] can't be encoded or decoded by [`bincode`].
pub trait GlobalId: Serialize + DeserializeOwned {
    const KEY: NodeKey;

    fn as_global_id(&self) -> Result<ID> {
        AnyGlobalId::from_node(self).and_then(|a| a.as_global_id())
    }

    fn from_global_id(id: &ID) -> Result<Self> {
        let any_global_id = AnyGlobalId::from_global_id(id)?;
        if any_global_id.key != Self::KEY {
            return Err(Error::invalid_argument(format!(
                "Expect {:?}, found {:?}",
                Self::KEY,
                any_global_id.key
            )));
        }
        Self::from_any_global_id_inner(&any_global_id.inner)
    }

    #[doc = "hidden"]
    fn to_any_global_id_inner(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| Error::internal(format!("Failed to encode node ID {e:?}")))
    }

    #[doc = "hidden"]
    fn from_any_global_id_inner(inner: &[u8]) -> Result<Self> {
        serde_json::from_slice::<Self>(inner)
            .map_err(|e| Error::invalid_argument(format!("Malformed inner bytes node {e:?}")))
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct AnyGlobalId {
    key: NodeKey,
    inner: Vec<u8>,
}

impl AnyGlobalId {
    fn from_global_id(id: &ID) -> Result<Self> {
        let bytes = BASE64_URL_SAFE_NO_PAD
            .decode(id.as_bytes())
            .map_err(|e| Error::invalid_argument(format!("Malformed global ID encode 1 {e:?}")))?;
        let (any_global_id, _) =
            bincode::decode_from_slice::<AnyGlobalId, _>(&bytes, config::standard()).map_err(
                |e| Error::invalid_argument(format!("Malformed global ID encode 2 {e:?}")),
            )?;
        Ok(any_global_id)
    }

    fn as_global_id(&self) -> Result<ID> {
        let bytes = bincode::encode_to_vec(self, config::standard())
            .map_err(|e| Error::internal(format!("Failed to encode global ID bytes {e:?}")))?;
        Ok(BASE64_URL_SAFE_NO_PAD.encode(bytes).into())
    }

    fn from_node<N: GlobalId>(node: &N) -> Result<Self> {
        Ok(Self {
            key: N::KEY,
            inner: node.to_any_global_id_inner()?,
        })
    }
}

impl GlobalId for Session {
    const KEY: NodeKey = NodeKey::Session;
}

impl GlobalId for Customer {
    const KEY: NodeKey = NodeKey::Customer;
}

impl GlobalId for Handyman {
    const KEY: NodeKey = NodeKey::Handyman;
}

impl GlobalId for CustomerProfile {
    const KEY: NodeKey = NodeKey::CustomerProfile;
}

impl GlobalId for HandymanProfile {
    const KEY: NodeKey = NodeKey::HandymanProfile;
}

pub fn parse_any_global_id(id: &ID) -> Result<Option<Node>> {
    let any_global_id = AnyGlobalId::from_global_id(id)?;
    let node = match any_global_id.key {
        NodeKey::Session => Node::Session(Session::from_any_global_id_inner(&any_global_id.inner)?),
        NodeKey::Customer => {
            Node::Customer(Customer::from_any_global_id_inner(&any_global_id.inner)?)
        }
        NodeKey::Handyman => {
            Node::Handyman(Handyman::from_any_global_id_inner(&any_global_id.inner)?)
        }
        NodeKey::CustomerProfile => Node::CustomerProfile(
            CustomerProfile::from_any_global_id_inner(&any_global_id.inner)?,
        ),
        NodeKey::HandymanProfile => Node::HandymanProfile(
            HandymanProfile::from_any_global_id_inner(&any_global_id.inner)?,
        ),
        #[allow(unreachable_patterns)]
        _ => return Ok(None),
    };
    Ok(Some(node))
}
