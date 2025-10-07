mod graphql;

use account_service_server::SESSION_TOKEN_TTL_DAYS;
use chrono::Duration;
use error::{ErrorVariant, Result};
use test_service_orchestration::{ServiceEnvironment, ServiceParams};

#[tokio::test]
async fn customer_registration() -> Result<()> {
    let ServiceEnvironment {
        _pg_container,
        core_service,
        ..
    } = ServiceParams::default().init().await?;
    let client =
        graphql::GraphqlClient::new(format!("http://{}/graphql", core_service.service_host));

    let phone_number = "+84334445555";
    let password = "12345678";

    // Test registration flow including create account & create profile
    graphql::customer_account_start_registration(&client, phone_number)
        .await
        .data
        .unwrap()
        .user_account_start_registration
        .case
        .try_otp_case()?;

    let sms = core_service.sms_receiver.receive_sms().await.pop().unwrap();
    let session = graphql::customer_account_finish_registration(
        &client,
        graphql::UserAccountFinishRegistrationInput {
            phone_number,
            password,
            otp_code: &sms.message.try_otp_verification_for_registration()?.code,
        },
    )
    .await
    .data
    .map(|d| d.user_account_finish_registration.session)
    .unwrap();
    let session_ttl = session.exp - session.iat;
    assert_eq!(session_ttl, Duration::days(SESSION_TOKEN_TTL_DAYS as i64));

    let customer = session.actor_type.try_customer()?;
    assert_eq!(customer.phone_number, phone_number);
    assert!(customer.profile.is_none());

    let nick_name = "Nguyễn Văn A";
    let profile = graphql::customer_create_profile(
        &client,
        graphql::customer_create_profile::CustomerCreateProfileInput {
            customer_id: customer.id.clone(),
            nick_name: nick_name.into(),
        },
    )
    .await
    .data
    .unwrap()
    .customer_create_profile
    .customer
    .profile
    .unwrap();
    assert_eq!(profile.nick_name, nick_name);

    // Test session is granted right after account creation (no need sign in)
    let session = graphql::session(&client)
        .await
        .data
        .unwrap()
        .session
        .unwrap();
    let customer = session.actor_type.try_customer()?;
    let profile = customer.profile.as_ref().unwrap();
    assert_eq!(customer.phone_number, phone_number);
    assert_eq!(profile.nick_name, nick_name);

    // Test client without session tokens having no access to user session
    client.clear_session().await;
    let session = graphql::session(&client).await.data.unwrap().session;
    assert!(session.is_none());

    // Test sign in with password
    graphql::user_sign_in_with_password(
        &client,
        graphql::user_sign_in_with_password::UserSignInWithPasswordInput {
            phone_number: phone_number.into(),
            password: password.into(),
            account_type: graphql::user_sign_in_with_password::AccountType::CUSTOMER,
        },
    )
    .await
    .data
    .unwrap();
    let session = graphql::session(&client)
        .await
        .data
        .unwrap()
        .session
        .unwrap();
    let customer = session.actor_type.try_customer()?;
    let profile = customer.profile.as_ref().unwrap();
    assert_eq!(customer.phone_number, phone_number);
    assert_eq!(profile.nick_name, nick_name);

    // Test sign in with password but wrong account type
    let response = graphql::user_sign_in_with_password(
        &client,
        graphql::user_sign_in_with_password::UserSignInWithPasswordInput {
            phone_number: phone_number.into(),
            password: password.into(),
            account_type: graphql::user_sign_in_with_password::AccountType::HANDYMAN,
        },
    )
    .await;
    graphql::assert_error_response(
        response,
        ErrorVariant::Unauthenticated(None),
        Some(serde_json::json!(null)),
    );

    Ok(())
}

#[tokio::test]
async fn handyman_registration() -> Result<()> {
    let ServiceEnvironment {
        _pg_container,
        core_service,
        ..
    } = ServiceParams::default().init().await?;
    let client =
        graphql::GraphqlClient::new(format!("http://{}/graphql", core_service.service_host));

    let phone_number = "+84334445555";
    let password = "12345678";

    // Test registration flow including create account & create profile
    graphql::handyman_account_start_registration(&client, phone_number)
        .await
        .data
        .unwrap()
        .user_account_start_registration
        .case
        .try_otp_case()?;

    let sms = core_service.sms_receiver.receive_sms().await.pop().unwrap();
    let session = graphql::handyman_account_finish_registration(
        &client,
        graphql::UserAccountFinishRegistrationInput {
            phone_number,
            password,
            otp_code: &sms.message.try_otp_verification_for_registration()?.code,
        },
    )
    .await
    .data
    .map(|d| d.user_account_finish_registration.session)
    .unwrap();
    let session_ttl = session.exp - session.iat;
    assert_eq!(session_ttl, Duration::days(SESSION_TOKEN_TTL_DAYS as i64));

    let handyman = session.actor_type.try_handyman()?;
    assert_eq!(handyman.phone_number, phone_number);
    assert!(handyman.profile.is_none());

    let first_name = "B";
    let last_name = "Nguyễn Văn";
    let profile = graphql::handyman_create_profile(
        &client,
        graphql::handyman_create_profile::HandymanCreateProfileInput {
            handyman_id: handyman.id.clone(),
            first_name: first_name.into(),
            last_name: last_name.into(),
        },
    )
    .await
    .data
    .unwrap()
    .handyman_create_profile
    .handyman
    .profile
    .unwrap();
    assert_eq!(profile.first_name, first_name);
    assert_eq!(profile.last_name, last_name);

    // Test session is granted right after account creation (no need sign in)
    let session = graphql::session(&client)
        .await
        .data
        .unwrap()
        .session
        .unwrap();
    let handyman = session.actor_type.try_handyman()?;
    let profile = handyman.profile.as_ref().unwrap();
    assert_eq!(handyman.phone_number, phone_number);
    assert_eq!(profile.first_name, first_name);
    assert_eq!(profile.last_name, last_name);

    // Test client without session tokens having no access to user session
    client.clear_session().await;
    let session = graphql::session(&client).await.data.unwrap().session;
    assert!(session.is_none());

    // Test sign in with password
    graphql::user_sign_in_with_password(
        &client,
        graphql::user_sign_in_with_password::UserSignInWithPasswordInput {
            phone_number: phone_number.into(),
            password: password.into(),
            account_type: graphql::user_sign_in_with_password::AccountType::HANDYMAN,
        },
    )
    .await
    .data
    .unwrap();
    let session = graphql::session(&client)
        .await
        .data
        .unwrap()
        .session
        .unwrap();
    let handyman = session.actor_type.try_handyman()?;
    let profile = handyman.profile.as_ref().unwrap();
    assert_eq!(handyman.phone_number, phone_number);
    assert_eq!(profile.first_name, first_name);
    assert_eq!(profile.last_name, last_name);

    // Test sign in with password but wrong account type
    let response = graphql::user_sign_in_with_password(
        &client,
        graphql::user_sign_in_with_password::UserSignInWithPasswordInput {
            phone_number: phone_number.into(),
            password: password.into(),
            account_type: graphql::user_sign_in_with_password::AccountType::CUSTOMER,
        },
    )
    .await;
    graphql::assert_error_response(
        response,
        ErrorVariant::Unauthenticated(None),
        Some(serde_json::json!(null)),
    );

    Ok(())
}
