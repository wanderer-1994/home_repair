//! Common error-space shared among all Yearnings crates
//!
//! Built out of GRPC standard status error-space combined with google apis error details,
//! which is proven to be sufficient for conveying errors between applications.
//! See <https://grpc.github.io/grpc/core/md_doc_statuscodes.html>
//! and <https://github.com/googleapis/googleapis/blob/master/google/rpc/error_details.proto>

use tracing_error::SpanTrace;

use crate::error_details::{
    BadRequest, DebugInfo, ErrorInfo, PreconditionFailure, QuotaFailure, ResourceInfo, RetryInfo,
};

#[derive(Debug)]
#[cfg_attr(feature = "cloneable", derive(Clone))]
pub enum ErrorVariant {
    /// The operation was cancelled, typically by the caller.
    Cancelled,

    /// Unknown error. For example, this error may be returned when a Status
    /// value received from another address space belongs to an error space that
    /// is not known in this address space.
    ///
    /// Also errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    Unknown(Option<DebugInfo>),

    /// The client specified an invalid argument. Note that this differs from
    /// [`Self::FailedPrecondition`].
    ///
    /// [`Self::InvalidArgument`] indicates arguments that are problematic
    /// regardless of the state of the system (e.g., a malformed file name).
    InvalidArgument(Option<BadRequest>),

    /// The deadline expired before the operation could complete.
    ///
    /// For operations that change the state of the system, this error may be
    /// returned even if the operation has completed successfully.
    ///
    /// For example, a successful response from a server could have been delayed
    /// long.
    DeadlineExceeded,

    /// Some requested entity (e.g., file or directory) was not found.
    ///
    /// Note to server developers: if a request is denied for an entire class of
    /// users, such as gradual feature rollout or undocumented allowlist,
    /// [`Self::NotFound`] may be used. If a request is denied for some users
    /// within a class of users, such as user-based access control,
    /// [`Self::PermissionDenied`] must be used.
    NotFound(Option<ResourceInfo>),

    /// The entity that a client attempted to create (e.g., file or directory)
    /// already exists.
    AlreadyExists(Option<ResourceInfo>),

    /// The caller does not have permission to execute the specified operation.
    ///
    /// [`Self::PermissionDenied`] must not be used for rejections caused by
    /// exhausting some resource (use [`Self::ResourceExhausted`] instead for
    /// those errors).
    ///
    /// [`Self::PermissionDenied`] must not be used if the caller can not be
    /// identified (use [`Self::Unauthenticated`] instead for those errors).
    ///
    /// This error code does not imply the request is valid or the requested
    /// entity exists or satisfies other pre-conditions.
    PermissionDenied(Option<ErrorInfo>),

    /// Some resource has been exhausted, perhaps a per-user quota, or perhaps
    /// the entire file system is out of space.
    ResourceExhausted(Option<QuotaFailure>),

    /// The operation was rejected because the system is not in a state required
    /// for the operation's execution.
    ///
    /// For example, the directory to be deleted is non-empty, an rmdir
    /// operation is applied to a non-directory, etc.
    ///
    /// Service implementors can use the following guidelines to decide between
    /// [`Self::FailedPrecondition`], [`Self::Aborted`], and
    /// [`Self::Unavailable`]:
    ///
    /// 1. Use [`Self::Unavailable`] if the client can retry just the failing
    ///    call.
    /// 2. Use [`Self::Aborted`] if the client should retry at a higher level
    ///    (e.g., when a client-specified test-and-set fails, indicating the
    ///    client should restart a read-modify-write sequence).
    /// 3. Use [`Self::FailedPrecondition`] if the client should not retry until
    ///    the system state has been explicitly fixed. E.g., if an "rmdir" fails
    ///    because the directory is non-empty, [`Self::FailedPrecondition`]
    ///    should be returned since the client should not retry unless the files
    ///    are deleted from the directory.
    FailedPrecondition(Option<PreconditionFailure>),

    /// The operation was aborted, typically due to a concurrency issue such as
    /// a sequencer check failure or transaction abort.
    ///
    /// See the guidelines above for deciding between
    /// [`Self::FailedPrecondition`], [`Self::Aborted`], and
    /// [`Self::Unavailable`].
    Aborted((Option<ErrorInfo>, Option<RetryInfo>)),

    /// The operation was attempted past the valid range.
    ///
    /// E.g., seeking or reading past end-of-file. Unlike
    /// [`Self::InvalidArgument`], this error indicates a problem that may be
    /// fixed if the system state changes.
    ///
    /// For example, a 32-bit file system will generate
    /// [`Self::InvalidArgument`] if asked to read at an offset that is not in
    /// the range [0,2^32-1], but it will generate [`Self::OutOfRange`] if
    /// asked to read from an offset past the current file size.
    ///
    /// There is a fair bit of overlap between [`Self::FailedPrecondition`] and
    /// [`Self::OutOfRange`].
    ///
    /// We recommend using [`Self::OutOfRange`] (the more specific error) when
    /// it applies so that callers who are iterating through a space can easily
    /// look for an [`Self::OutOfRange`] error to detect when they are done.
    OutOfRange(Option<BadRequest>),

    /// The operation is not implemented or is not supported/enabled in this
    /// service.
    Unimplemented,

    /// Internal errors. This means that some invariants expected by the
    /// underlying system have been broken. This error code is reserved for
    /// serious errors.
    Internal(Option<DebugInfo>),

    /// The service is currently unavailable. This is most likely a transient
    /// condition, which can be corrected by retrying with a backoff. Note that
    /// it is not always safe to retry non-idempotent operations.
    Unavailable((Option<DebugInfo>, Option<RetryInfo>)),

    /// Unrecoverable data loss or corruption.
    DataLoss(Option<DebugInfo>),

    /// The request does not have valid authentication credentials for the
    /// operation.
    Unauthenticated(Option<ErrorInfo>),
}

impl ErrorVariant {
    /// Code that is provided to client applications.
    pub fn client_code(&self) -> &'static str {
        match self {
            Self::Cancelled => "CANCELLED",
            Self::Unknown(_) => "UNKNOWN",
            Self::InvalidArgument(_) => "INVALID_ARGUMENT",
            Self::DeadlineExceeded => "DEADLINE_EXCEEDED",
            Self::NotFound(_) => "NOT_FOUND",
            Self::AlreadyExists(_) => "ALREADY_EXISTS",
            Self::PermissionDenied(_) => "PERMISSION_DENIED",
            Self::ResourceExhausted(_) => "RESOURCE_EXHAUSTED",
            Self::FailedPrecondition(_) => "FAILED_PRECONDITION",
            Self::Aborted(_) => "ABORTED",
            Self::OutOfRange(_) => "OUT_OF_RANGE",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::Internal(_) => "INTERNAL",
            Self::Unavailable(_) => "UNAVAILABLE",
            Self::DataLoss(_) => "DATA_LOSS",
            Self::Unauthenticated(_) => "UNAUTHENTICATED",
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "cloneable", derive(Clone))]
/// Error type used across yearnings crates
pub struct Error {
    pub message: String,
    pub variant: Box<ErrorVariant>,
    pub span_trace: SpanTrace,
}

/// Result type used across yearnings crates
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn client_code(&self) -> &'static str {
        self.variant.client_code()
    }
}

// Implement utility functions for constructing Error
impl Error {
    pub fn new<S: Into<String>>(message: S, variant: ErrorVariant) -> Self {
        Self {
            message: message.into(),
            variant: Box::new(variant),
            span_trace: SpanTrace::capture(),
        }
    }

    pub fn cancelled<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::Cancelled)
    }

    pub fn unknown<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::Unknown(Some(DebugInfo::collect())))
    }

    pub fn unknown_with<S: Into<String>>(message: S, debug_info: Option<DebugInfo>) -> Self {
        Error::new(message, ErrorVariant::Unknown(debug_info))
    }

    pub fn invalid_argument<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::InvalidArgument(None))
    }

    pub fn invalid_argument_with<S: Into<String>>(
        message: S,
        bad_request: Option<BadRequest>,
    ) -> Self {
        Error::new(message, ErrorVariant::InvalidArgument(bad_request))
    }

    pub fn deadline_exceeded<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::DeadlineExceeded)
    }

    pub fn not_found<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::NotFound(None))
    }

    pub fn not_found_with<S: Into<String>>(
        message: S,
        resource_info: Option<ResourceInfo>,
    ) -> Self {
        Error::new(message, ErrorVariant::NotFound(resource_info))
    }

    pub fn already_exists<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::AlreadyExists(None))
    }

    pub fn already_exists_with<S: Into<String>>(
        message: S,
        resource_info: Option<ResourceInfo>,
    ) -> Self {
        Error::new(message, ErrorVariant::AlreadyExists(resource_info))
    }

    pub fn permission_denied<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::PermissionDenied(None))
    }

    pub fn permission_denied_with<S: Into<String>>(
        message: S,
        error_info: Option<ErrorInfo>,
    ) -> Self {
        Error::new(message, ErrorVariant::PermissionDenied(error_info))
    }

    pub fn resource_exhausted<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::ResourceExhausted(None))
    }

    pub fn resource_exhausted_with<S: Into<String>>(
        message: S,
        quota_failure: Option<QuotaFailure>,
    ) -> Self {
        Error::new(message, ErrorVariant::ResourceExhausted(quota_failure))
    }

    pub fn failed_precondition<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::FailedPrecondition(None))
    }

    pub fn failed_precondition_with<S: Into<String>>(
        message: S,
        precondition_failure: Option<PreconditionFailure>,
    ) -> Self {
        Error::new(
            message,
            ErrorVariant::FailedPrecondition(precondition_failure),
        )
    }

    pub fn aborted<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::Aborted((None, None)))
    }

    pub fn aborted_with<S: Into<String>>(
        message: S,
        error_info: Option<ErrorInfo>,
        retry_info: Option<RetryInfo>,
    ) -> Self {
        Error::new(message, ErrorVariant::Aborted((error_info, retry_info)))
    }

    pub fn out_of_range<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::OutOfRange(None))
    }

    pub fn out_of_range_with<S: Into<String>>(message: S, bad_request: Option<BadRequest>) -> Self {
        Error::new(message, ErrorVariant::OutOfRange(bad_request))
    }

    pub fn unimplemented<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::Unimplemented)
    }

    pub fn internal<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::Internal(None))
    }

    pub fn internal_with<S: Into<String>>(message: S, debug_info: Option<DebugInfo>) -> Self {
        Error::new(message, ErrorVariant::Internal(debug_info))
    }

    pub fn unavailable<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::Unavailable((None, None)))
    }

    pub fn unavailable_with<S: Into<String>>(
        message: S,
        debug_info: Option<DebugInfo>,
        retry_info: Option<RetryInfo>,
    ) -> Self {
        Error::new(message, ErrorVariant::Unavailable((debug_info, retry_info)))
    }

    pub fn data_loss<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::DataLoss(None))
    }

    pub fn data_loss_with<S: Into<String>>(message: S, debug_info: Option<DebugInfo>) -> Self {
        Error::new(message, ErrorVariant::DataLoss(debug_info))
    }

    pub fn unauthenticated<S: Into<String>>(message: S) -> Self {
        Error::new(message, ErrorVariant::Unauthenticated(None))
    }

    pub fn unauthenticated_with<S: Into<String>>(
        message: S,
        error_info: Option<ErrorInfo>,
    ) -> Self {
        Error::new(message, ErrorVariant::Unauthenticated(error_info))
    }

    /// This will cause the error to be logged with sentry error when the sentry layer is in the subscriber.
    pub fn trace(&self) {
        tracing::error!(
            code=self.client_code(),
            span_trace=%self.span_trace,
            variant_payload = ?self.variant,
            "{}", self.message
        );
    }
}
