use account_service_server::{
    AccountExistsRequest, CustomerCreateProfileRequest, CustomerRegisterRequest,
    CustomerSignInWithPasswordRequest, HandymanCreateProfileRequest, HandymanRegisterRequest,
    HandymanSigninWithPasswordRequest,
};
use async_graphql::{Context, ID, InputObject, Object, SimpleObject, Union};
use core_service_graphql_context::RequestContext;
use core_service_graphql_types::{Customer, GlobalId, Handyman, Session};
use entity_type::AccountType;
use error::Result;
use sms_sender::{MessageType, OtpVerificationForRegistration, SendSmsInput};
use std::sync::Arc;

#[derive(Default)]
pub struct SignUpAndAuthMutation;

#[Object]
impl SignUpAndAuthMutation {
    #[tracing::instrument(skip(self, ctx))]
    async fn user_account_start_registration(
        &self,
        ctx: &Context<'_>,
        input: UserAccountStartRegistrationInput,
    ) -> Result<UserAccountStartRegistrationPayload> {
        let UserAccountStartRegistrationInput {
            phone_number,
            account_type,
        } = input;
        let phone_number = typesafe::phone_number_from_str(&phone_number)?;
        let e164_phone_number_str = typesafe::phone_number_to_e164_format(&phone_number);
        let context = ctx.data::<RequestContext>()?;

        let exists = context
            .account_service_client
            .account_exists(AccountExistsRequest {
                e164_phone_number_str: e164_phone_number_str.clone(),
                account_type,
            })
            .await?
            .exists;

        if exists {
            return Ok(UserAccountStartRegistrationPayload {
                case: StartRegistrationCase::AccountExist(StartRegistrationCaseAccountExist {
                    foo: true,
                }),
            });
        }

        let otp_code = context
            .pending_registration_phone_cache(e164_phone_number_str.clone())
            .await?;

        context
            .sms_sender
            .send(SendSmsInput {
                to: phone_number,
                message: MessageType::OtpVerificationForRegistration(
                    OtpVerificationForRegistration {
                        code: otp_code.code,
                    },
                ),
            })
            .await?;

        Ok(UserAccountStartRegistrationPayload {
            case: StartRegistrationCase::OtpCode(StartRegistrationCaseOtpCode {
                digits: otp_code.digits,
                ttl_seconds: otp_code.ttl_seconds,
                e164_phone_number_str,
            }),
        })
    }

    #[tracing::instrument(skip(self, ctx))]
    async fn user_account_finish_registration(
        &self,
        ctx: &Context<'_>,
        input: UserAccountFinishRegistrationInput,
    ) -> Result<UserAccountFinishRegistrationPayload> {
        let UserAccountFinishRegistrationInput {
            phone_number,
            password,
            otp_code,
            account_type,
        } = input;
        let phone_number = typesafe::phone_number_from_str(&phone_number)?;
        let e164_phone_number_str = typesafe::phone_number_to_e164_format(&phone_number);

        let context = ctx.data::<RequestContext>()?;
        context
            .pending_registration_phone_validate(&e164_phone_number_str, &otp_code)
            .await?;

        let initiate_session = match account_type {
            AccountType::Customer => {
                context
                    .account_service_client
                    .customer_register(CustomerRegisterRequest {
                        e164_phone_number_str,
                        password,
                    })
                    .await?
                    .initiate_session
            }
            AccountType::Handyman => {
                context
                    .account_service_client
                    .handyman_register(HandymanRegisterRequest {
                        e164_phone_number_str,
                        password,
                    })
                    .await?
                    .initiate_session
            }
        };

        let session = context.set_session(ctx, initiate_session).await?;
        Ok(UserAccountFinishRegistrationPayload {
            session: Session::new(session),
        })
    }

    #[tracing::instrument(skip_all, fields(phone_number = input.phone_number.as_str()))]
    async fn user_sign_in_with_password(
        &self,
        ctx: &Context<'_>,
        input: UserSignInWithPasswordInput,
    ) -> Result<UserSignInWithPasswordPayload> {
        let UserSignInWithPasswordInput {
            phone_number,
            password,
            account_type,
        } = input;
        let e164_phone_number_str = typesafe::normalize_phone_number_str(&phone_number)?;
        let context = ctx.data::<RequestContext>()?;

        let initiate_session = match account_type {
            AccountType::Customer => {
                context
                    .account_service_client
                    .customer_sign_in_with_password(CustomerSignInWithPasswordRequest {
                        e164_phone_number_str,
                        password,
                    })
                    .await?
                    .initiate_session
            }
            AccountType::Handyman => {
                context
                    .account_service_client
                    .handyman_sign_in_with_password(HandymanSigninWithPasswordRequest {
                        e164_phone_number_str,
                        password,
                    })
                    .await?
                    .initiate_session
            }
        };

        let session = context.set_session(ctx, initiate_session).await?;
        Ok(UserSignInWithPasswordPayload {
            session: Session::new(session),
        })
    }

    #[tracing::instrument(skip(self, ctx))]
    async fn customer_create_profile(
        &self,
        ctx: &Context<'_>,
        input: CustomerCreateProfileInput,
    ) -> Result<CustomerCreateProfilePayload> {
        let CustomerCreateProfileInput {
            customer_id,
            nick_name,
        } = input;
        let customer_id = Customer::from_global_id(&customer_id)?.inner_id();
        let context = ctx.data::<RequestContext>()?;
        let req_session = context.try_session_context().await?;

        let profile = context
            .account_service_client
            .customer_create_profile(CustomerCreateProfileRequest {
                actor_auth: req_session.as_actor_auth(),
                customer_id,
                nick_name,
            })
            .await?
            .profile;

        context
            .customer_loaders
            .profile_by_id_loader
            .feed_one(profile.customer_id, Arc::new(profile))
            .await;

        Ok(CustomerCreateProfilePayload {
            customer: Customer::new(customer_id),
        })
    }

    #[tracing::instrument(skip(self, ctx))]
    async fn handyman_create_profile(
        &self,
        ctx: &Context<'_>,
        input: HandymanCreateProfileInput,
    ) -> Result<HandymanCreateProfilePayload> {
        let HandymanCreateProfileInput {
            handyman_id,
            first_name,
            last_name,
        } = input;
        let handyman_id = Handyman::from_global_id(&handyman_id)?.inner_id();
        let context = ctx.data::<RequestContext>()?;
        let req_session = context.try_session_context().await?;

        let profile = context
            .account_service_client
            .handyman_create_profile(HandymanCreateProfileRequest {
                actor_auth: req_session.as_actor_auth(),
                handyman_id,
                first_name,
                last_name,
            })
            .await?
            .profile;

        context
            .handyman_loaders
            .profile_by_id_loader
            .feed_one(profile.handyman_id, Arc::new(profile))
            .await;

        Ok(HandymanCreateProfilePayload {
            handyman: Handyman::new(handyman_id),
        })
    }
}

#[derive(Debug, InputObject)]
struct UserAccountStartRegistrationInput {
    phone_number: String,
    /// Improves UX by pre-checking the existence of account type associated with the phone number.
    account_type: AccountType,
}

#[derive(SimpleObject)]
struct UserAccountStartRegistrationPayload {
    case: StartRegistrationCase,
}

#[derive(Union)]
enum StartRegistrationCase {
    AccountExist(StartRegistrationCaseAccountExist),
    OtpCode(StartRegistrationCaseOtpCode),
}

#[derive(SimpleObject)]
/// Indicates there already exists an account with same phone number,
/// user should sign up or use a different phone number.
struct StartRegistrationCaseAccountExist {
    /// Dummy field, always `true`
    foo: bool,
}

#[derive(SimpleObject)]
/// Indicates an OTP code has been sent to user phone number (zalo) for verification.
struct StartRegistrationCaseOtpCode {
    digits: u8,
    ttl_seconds: u64,
    /// The standard phone number format captured by backend
    e164_phone_number_str: String,
}

#[derive(Debug, InputObject)]
struct UserAccountFinishRegistrationInput {
    phone_number: String,
    password: String,
    otp_code: String,
    account_type: AccountType,
}

#[derive(SimpleObject)]
struct UserAccountFinishRegistrationPayload {
    session: Session,
}

#[derive(InputObject)]
struct UserSignInWithPasswordInput {
    phone_number: String,
    password: String,
    account_type: AccountType,
}

#[derive(SimpleObject)]
struct UserSignInWithPasswordPayload {
    session: Session,
}

#[derive(Debug, InputObject)]
struct CustomerCreateProfileInput {
    /// Requires customer_id to allow admin control.
    /// Follow this convention for all other apis.
    customer_id: ID,
    nick_name: String,
}

#[derive(SimpleObject)]
struct CustomerCreateProfilePayload {
    customer: Customer,
}

#[derive(Debug, InputObject)]
struct HandymanCreateProfileInput {
    handyman_id: ID,
    first_name: String,
    last_name: String,
}

#[derive(SimpleObject)]
struct HandymanCreateProfilePayload {
    handyman: Handyman,
}
