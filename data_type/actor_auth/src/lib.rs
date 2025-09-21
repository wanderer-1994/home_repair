use chrono::{DateTime, NaiveDateTime};
use entity_type::{CustomerId, HandymanId};
use error::{
    Error, Result, assert_argument_is_some,
    error_details::{BadRequest, bad_request::FieldViolation},
};

pub mod proto {
    tonic::include_proto!("actor_auth");
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CustomerActor {
    pub customer_id: CustomerId,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HandymanActor {
    pub handyman_id: HandymanId,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "account_type"))]
pub enum ActorType {
    Customer(CustomerActor),
    Handyman(HandymanActor),
}

impl From<ActorType> for proto::ActorType {
    fn from(value: ActorType) -> Self {
        use proto::{CustomerActor, HandymanActor, actor_type::Inner};

        let inner = match value {
            ActorType::Customer(c) => Inner::Customer(CustomerActor {
                customer_id: c.customer_id.0,
            }),
            ActorType::Handyman(h) => Inner::Handyman(HandymanActor {
                handyman_id: h.handyman_id.0,
            }),
        };

        proto::ActorType { inner: Some(inner) }
    }
}

impl TryFrom<proto::ActorType> for ActorType {
    type Error = Error;

    fn try_from(value: proto::ActorType) -> Result<Self> {
        use proto::actor_type::Inner;

        match value.inner {
            Some(Inner::Customer(c)) => Ok(ActorType::Customer(CustomerActor {
                customer_id: CustomerId(c.customer_id),
            })),
            Some(Inner::Handyman(h)) => Ok(ActorType::Handyman(HandymanActor {
                handyman_id: HandymanId(h.handyman_id),
            })),
            None => Err(Error::invalid_argument_with(
                "Missing session type inner",
                Some(BadRequest {
                    field_violations: vec![FieldViolation {
                        field: String::from("session_type_inner"),
                        description: String::from("missing"),
                    }],
                }),
            )),
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub iat: NaiveDateTime,
    pub exp: NaiveDateTime,
    pub actor_type: ActorType,
}

impl From<Session> for proto::Session {
    fn from(value: Session) -> Self {
        proto::Session {
            iat: value.iat.and_utc().timestamp(),
            exp: value.exp.and_utc().timestamp(),
            actor_type: Some(proto::ActorType::from(value.actor_type)),
        }
    }
}

impl TryFrom<proto::Session> for Session {
    type Error = Error;

    fn try_from(
        proto::Session {
            iat,
            exp,
            actor_type,
        }: proto::Session,
    ) -> Result<Self> {
        assert_argument_is_some!(actor_type);

        Ok(Self {
            iat: timestamp_to_naive_date_time(iat)?,
            exp: timestamp_to_naive_date_time(exp)?,
            actor_type: ActorType::try_from(actor_type)?,
        })
    }
}

#[derive(Debug)]
pub enum ActorAuth {
    God,
    Session(Session),
}

impl From<ActorAuth> for proto::ActorAuth {
    fn from(value: ActorAuth) -> Self {
        use proto::actor_auth::Inner;

        let inner = match value {
            ActorAuth::God => Inner::God(()),
            ActorAuth::Session(s) => Inner::Session(proto::Session::from(s)),
        };

        proto::ActorAuth { inner: Some(inner) }
    }
}

impl TryFrom<proto::ActorAuth> for ActorAuth {
    type Error = Error;

    fn try_from(value: proto::ActorAuth) -> Result<ActorAuth> {
        use proto::actor_auth::Inner;

        match value.inner {
            Some(Inner::God(_)) => Ok(ActorAuth::God),
            Some(Inner::Session(s)) => Ok(ActorAuth::Session(Session::try_from(s)?)),
            None => Err(Error::invalid_argument_with(
                "Missing actor auth inner",
                Some(BadRequest {
                    field_violations: vec![FieldViolation {
                        field: String::from("actor_auth_inner"),
                        description: String::from("missing"),
                    }],
                }),
            )),
        }
    }
}

impl ActorAuth {
    fn actor_type(&self) -> Option<&ActorType> {
        match self {
            ActorAuth::God => None,
            ActorAuth::Session(session) => Some(&session.actor_type),
        }
    }

    /// Check if the actor is the system.
    pub fn is_god(&self) -> bool {
        matches!(self, ActorAuth::God)
    }

    /// Check if the actor is admin account.
    /// N/B: in future, we might support admin account.
    pub fn is_admin(&self) -> bool {
        false
    }

    pub fn try_customer(&self) -> Result<&CustomerActor> {
        match self.actor_type() {
            Some(ActorType::Customer(inner)) => Ok(inner),
            _ => Err(Error::permission_denied("Unauthorized")),
        }
    }

    pub fn try_handyman(&self) -> Result<&HandymanActor> {
        match self.actor_type() {
            Some(ActorType::Handyman(inner)) => Ok(inner),
            _ => Err(Error::permission_denied("Unauthorized")),
        }
    }

    pub fn is_handyman(&self) -> bool {
        matches!(self.actor_type(), Some(ActorType::Handyman(_)))
    }

    pub fn require_god_or_admin(&self) -> Result<()> {
        if !self.is_god() && !self.is_admin() {
            return Err(Error::permission_denied("Unauthorized"));
        }
        Ok(())
    }

    pub fn require_customer_access(&self, customer_id: CustomerId) -> Result<()> {
        if self.require_god_or_admin().is_ok() {
            return Ok(());
        }

        let customer = self.try_customer()?;
        if customer.customer_id != customer_id {
            return Err(Error::permission_denied("Unauthorized"));
        }

        Ok(())
    }

    pub fn require_handyman_access(&self, handyman_id: HandymanId) -> Result<()> {
        if self.require_god_or_admin().is_ok() {
            return Ok(());
        }

        let handyman = self.try_handyman()?;
        if handyman.handyman_id != handyman_id {
            return Err(Error::permission_denied("Unauthorized"));
        }

        Ok(())
    }
}

fn timestamp_to_naive_date_time(timestamp: i64) -> Result<NaiveDateTime> {
    DateTime::from_timestamp_secs(timestamp)
        .ok_or_else(|| Error::internal(format!("Invalid timestamp {timestamp}")))
        .map(|d| d.naive_utc())
}
