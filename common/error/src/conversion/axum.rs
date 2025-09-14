//! Convert yearnings error space to axum response

use crate::{Error, ErrorVariant};
use async_graphql::ErrorExtensionValues;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match self.variant.as_ref() {
            ErrorVariant::Cancelled => {
                StatusCode::from_u16(499).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            ErrorVariant::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorVariant::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            ErrorVariant::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
            ErrorVariant::NotFound(_) => StatusCode::NOT_FOUND,
            ErrorVariant::AlreadyExists(_) => StatusCode::CONFLICT,
            ErrorVariant::PermissionDenied(_) => StatusCode::FORBIDDEN,
            ErrorVariant::ResourceExhausted(_) => StatusCode::TOO_MANY_REQUESTS,
            ErrorVariant::FailedPrecondition(_) => StatusCode::PRECONDITION_FAILED,
            ErrorVariant::Aborted(_) => StatusCode::CONFLICT,
            ErrorVariant::OutOfRange(_) => StatusCode::BAD_REQUEST,
            ErrorVariant::Unimplemented => StatusCode::NOT_IMPLEMENTED,
            ErrorVariant::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorVariant::Unavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ErrorVariant::DataLoss(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorVariant::Unauthenticated(_) => StatusCode::UNAUTHORIZED,
        };

        let json = ErrorExtensionValues::from(self);
        let json = serde_json::to_value(json).unwrap_or_default();

        (status_code, Json(json)).into_response()
    }
}
