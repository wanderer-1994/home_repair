#![allow(dead_code)]

use axum::http::HeaderName;
use graphql_client::{GraphQLQuery, Response};
use headers::{Cookie, Header};
use reqwest::{
    Client, StatusCode,
    header::{COOKIE, HeaderMap, HeaderValue, ORIGIN, SET_COOKIE, USER_AGENT},
};
use service_http::{ACCESS_TOKEN_COOKIE_KEY, CSRF_TOKEN_COOKIE_KEY, CSRF_TOKEN_HEADER_KEY};
use std::{collections::HashMap, str::FromStr};
use test_service_orchestration::core_service::TEST_ORIGIN;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub struct SessionTokens {
    pub access_token: String,
    pub csrf_token: String,
}

pub struct GraphqlClient {
    client: Client,
    graphql_url: String,
    session: Mutex<Option<SessionTokens>>,
    custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
}

impl GraphqlClient {
    pub fn new(graphql_url: String) -> Self {
        let client = Client::new();
        Self {
            client,
            graphql_url,
            session: Mutex::new(None),
            custom_headers: None,
        }
    }

    pub fn new_with_user_agent(graphql_url: String, user_agent: String) -> Self {
        let client = Client::new();
        Self {
            client,
            graphql_url,
            session: Mutex::new(None),
            custom_headers: Some(HashMap::from([(
                USER_AGENT,
                HeaderValue::from_str(&user_agent).unwrap(),
            )])),
        }
    }
    pub fn new_with_custom_headers(graphql_url: String, headers: HashMap<&str, &str>) -> Self {
        let client = Client::new();
        Self {
            client,
            graphql_url,
            session: Mutex::new(None),
            custom_headers: Some(
                headers
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            HeaderName::from_str(k).unwrap(),
                            HeaderValue::from_str(v).unwrap(),
                        )
                    })
                    .collect(),
            ),
        }
    }

    pub async fn current_session(&self) -> Option<SessionTokens> {
        self.session.lock().await.as_ref().cloned()
    }

    pub async fn clear_session(&self) {
        self.set_session(None).await;
    }

    pub async fn set_session(&self, session: Option<SessionTokens>) {
        let mut current_session = self.session.lock().await;
        *current_session = session;
    }

    async fn session_headers(&self) -> HeaderMap {
        let mut header_map = HeaderMap::with_capacity(3);
        let session = self.session.lock().await;
        if let Some(session) = session.as_ref() {
            header_map.append(
                CSRF_TOKEN_HEADER_KEY,
                HeaderValue::from_str(&session.csrf_token).unwrap(),
            );
            header_map.append(
                COOKIE,
                HeaderValue::from_str(&format!(
                    "{}={}",
                    ACCESS_TOKEN_COOKIE_KEY, session.access_token
                ))
                .unwrap(),
            );
            header_map.append(ORIGIN, HeaderValue::from_str(TEST_ORIGIN).unwrap());
        }

        if let Some(custom_headers) = self.custom_headers.as_ref() {
            for (key, value) in custom_headers {
                header_map.append(key.clone(), value.clone());
            }
        }

        header_map
    }

    pub async fn send_query<Q>(&self, variables: Q::Variables) -> Response<Q::ResponseData>
    where
        Q: GraphQLQuery,
    {
        let body = Q::build_query(variables);

        let response = self
            .client
            .post(&self.graphql_url)
            .headers(self.session_headers().await)
            .json(&body)
            .send()
            .await
            .unwrap();

        if response.status() != StatusCode::OK {
            panic!("Failed to make request to graphql server");
        }

        let mut cookies_iter = response.headers().get_all(SET_COOKIE).iter();

        let cookies = Cookie::decode(&mut cookies_iter).unwrap();

        if let Some((access_token, csrf_token)) = cookies
            .get(ACCESS_TOKEN_COOKIE_KEY)
            .zip(cookies.get(CSRF_TOKEN_COOKIE_KEY))
        {
            if access_token.is_empty() {
                self.clear_session().await;
            } else {
                self.set_session(Some(SessionTokens {
                    access_token: access_token.into(),
                    csrf_token: csrf_token.into(),
                }))
                .await;
            }
        }

        response.json::<Response<Q::ResponseData>>().await.unwrap()
    }
}
