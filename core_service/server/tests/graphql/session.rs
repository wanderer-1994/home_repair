#![allow(dead_code)]

use super::GraphqlClient;
use chrono::NaiveDateTime;
use error::{Error, Result};
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "tests/graphql/session.graphql",
    response_derives = "Debug"
)]
pub struct Session;

pub async fn session(client: &GraphqlClient) -> Response<session::ResponseData> {
    client.send_query::<Session>(session::Variables {}).await
}

impl session::SessionFragmentActorType {
    pub fn try_customer(&self) -> Result<&session::SessionFragmentActorTypeOnCustomer> {
        match self {
            session::SessionFragmentActorType::Customer(inner) => Ok(inner),
            session::SessionFragmentActorType::Handyman(_) => {
                Err(Error::internal("Not a customer session"))
            }
        }
    }

    pub fn try_handyman(&self) -> Result<&session::SessionFragmentActorTypeOnHandyman> {
        match self {
            session::SessionFragmentActorType::Customer(_) => {
                Err(Error::internal("Not a handyman session"))
            }
            session::SessionFragmentActorType::Handyman(inner) => Ok(inner),
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "tests/graphql/session.graphql",
    response_derives = "Debug"
)]
pub struct UserSignInWithPassword;

pub async fn user_sign_in_with_password(
    client: &GraphqlClient,
    input: user_sign_in_with_password::UserSignInWithPasswordInput,
) -> Response<user_sign_in_with_password::ResponseData> {
    client
        .send_query::<UserSignInWithPassword>(user_sign_in_with_password::Variables {
            input1: input,
        })
        .await
}
