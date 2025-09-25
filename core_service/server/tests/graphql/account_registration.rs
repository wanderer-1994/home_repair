#![allow(dead_code)]

use super::GraphqlClient;
use chrono::NaiveDateTime;
use error::{Error, Result};
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "tests/graphql/account_registration.graphql",
    response_derives = "Debug"
)]
pub struct UserAccountStartRegistration;

async fn user_account_start_registration(
    client: &GraphqlClient,
    phone_number: &str,
    account_type: user_account_start_registration::AccountType,
) -> Response<user_account_start_registration::ResponseData> {
    client
        .send_query::<UserAccountStartRegistration>(user_account_start_registration::Variables {
            input1: user_account_start_registration::UserAccountStartRegistrationInput {
                phone_number: phone_number.into(),
                account_type,
            },
        })
        .await
}

impl user_account_start_registration::UserAccountStartRegistrationUserAccountStartRegistrationCase {
    /// Return the OTP case immediately.
    pub fn try_otp_case(self) -> Result<user_account_start_registration::UserAccountStartRegistrationUserAccountStartRegistrationCaseOnStartRegistrationCaseOtpCode>{
        match self {
            user_account_start_registration::UserAccountStartRegistrationUserAccountStartRegistrationCase::StartRegistrationCaseAccountExist(_) => Err(Error::internal("Account already exist")),
            user_account_start_registration::UserAccountStartRegistrationUserAccountStartRegistrationCase::StartRegistrationCaseOtpCode(otp) => Ok(otp),
        }
    }
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "tests/graphql/account_registration.graphql",
    response_derives = "Debug"
)]
pub struct UserAccountFinishRegistration;

async fn user_account_finish_registration(
    client: &GraphqlClient,
    input: user_account_finish_registration::UserAccountFinishRegistrationInput,
) -> Response<user_account_finish_registration::ResponseData> {
    client
        .send_query::<UserAccountFinishRegistration>(user_account_finish_registration::Variables {
            input2: input,
        })
        .await
}

impl user_account_finish_registration::SessionFragmentActorType {
    pub fn try_customer(
        &self,
    ) -> Result<&user_account_finish_registration::SessionFragmentActorTypeOnCustomer> {
        match self {
            user_account_finish_registration::SessionFragmentActorType::Customer(inner) => {
                Ok(inner)
            }
            user_account_finish_registration::SessionFragmentActorType::Handyman(_) => {
                Err(Error::internal("Not a customer session"))
            }
        }
    }

    pub fn try_handyman(
        &self,
    ) -> Result<&user_account_finish_registration::SessionFragmentActorTypeOnHandyman> {
        match self {
            user_account_finish_registration::SessionFragmentActorType::Customer(_) => {
                Err(Error::internal("Not a handyman session"))
            }
            user_account_finish_registration::SessionFragmentActorType::Handyman(inner) => {
                Ok(inner)
            }
        }
    }
}

pub struct UserAccountFinishRegistrationInput<'a> {
    pub phone_number: &'a str,
    pub password: &'a str,
    pub otp_code: &'a str,
}

pub async fn customer_account_start_registration(
    client: &GraphqlClient,
    phone_number: &str,
) -> Response<user_account_start_registration::ResponseData> {
    user_account_start_registration(
        client,
        phone_number,
        user_account_start_registration::AccountType::CUSTOMER,
    )
    .await
}

pub async fn customer_account_finish_registration(
    client: &GraphqlClient,
    UserAccountFinishRegistrationInput {
        phone_number,
        password,
        otp_code,
    }: UserAccountFinishRegistrationInput<'_>,
) -> Response<user_account_finish_registration::ResponseData> {
    user_account_finish_registration(
        client,
        user_account_finish_registration::UserAccountFinishRegistrationInput {
            phone_number: phone_number.into(),
            password: password.into(),
            otp_code: otp_code.into(),
            account_type: user_account_finish_registration::AccountType::CUSTOMER,
        },
    )
    .await
}

pub async fn handyman_account_start_registration(
    client: &GraphqlClient,
    phone_number: &str,
) -> Response<user_account_start_registration::ResponseData> {
    user_account_start_registration(
        client,
        phone_number,
        user_account_start_registration::AccountType::HANDYMAN,
    )
    .await
}

pub async fn handyman_account_finish_registration(
    client: &GraphqlClient,
    UserAccountFinishRegistrationInput {
        phone_number,
        password,
        otp_code,
    }: UserAccountFinishRegistrationInput<'_>,
) -> Response<user_account_finish_registration::ResponseData> {
    user_account_finish_registration(
        client,
        user_account_finish_registration::UserAccountFinishRegistrationInput {
            phone_number: phone_number.into(),
            password: password.into(),
            otp_code: otp_code.into(),
            account_type: user_account_finish_registration::AccountType::HANDYMAN,
        },
    )
    .await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "tests/graphql/account_registration.graphql",
    response_derives = "Debug"
)]
pub struct CustomerCreateProfile;

pub async fn customer_create_profile(
    client: &GraphqlClient,
    input: customer_create_profile::CustomerCreateProfileInput,
) -> Response<customer_create_profile::ResponseData> {
    client
        .send_query::<CustomerCreateProfile>(customer_create_profile::Variables { input3: input })
        .await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "tests/graphql/account_registration.graphql",
    response_derives = "Debug"
)]
pub struct HandymanCreateProfile;

pub async fn handyman_create_profile(
    client: &GraphqlClient,
    input: handyman_create_profile::HandymanCreateProfileInput,
) -> Response<handyman_create_profile::ResponseData> {
    client
        .send_query::<HandymanCreateProfile>(handyman_create_profile::Variables { input4: input })
        .await
}
