use crate::{Customer, GlobalId, Handyman};
use actor_auth::{ActorKey, Session as ActorSession};
use async_graphql::{ID, Object, Union};
use chrono::NaiveDateTime;
use error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub key: ActorKey,
    #[serde(skip, default = "Option::default")]
    session: Option<Arc<ActorSession>>,
}

impl Session {
    pub fn new(actor_session: Arc<ActorSession>) -> Self {
        Self {
            key: ActorKey::from(&actor_session.actor_type),
            session: Some(actor_session),
        }
    }

    fn get(&self) -> Result<&ActorSession> {
        self.session
            .as_deref()
            .ok_or_else(|| Error::internal("Session is initiated with non session"))
    }
}

#[Object]
impl Session {
    pub async fn id(&self) -> Result<ID> {
        self.as_global_id()
    }

    async fn iat(&self) -> Result<NaiveDateTime> {
        Ok(self.get()?.exp)
    }

    async fn exp(&self) -> Result<NaiveDateTime> {
        Ok(self.get()?.exp)
    }

    async fn actor_type(&self) -> Result<ActorType> {
        match self.get()?.actor_type {
            actor_auth::ActorType::Customer(actor) => {
                Ok(ActorType::Customer(Customer::new(actor.customer_id)))
            }
            actor_auth::ActorType::Handyman(actor) => {
                Ok(ActorType::Handyman(Handyman::new(actor.handyman_id)))
            }
        }
    }
}

#[derive(Union)]
enum ActorType {
    Customer(Customer),
    Handyman(Handyman),
}
