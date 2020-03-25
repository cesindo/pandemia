// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

//! The set of errors for the API module.

use validator::{ValidationErrors, ValidationErrorsKind};

use crate::{api::ApiResult, error::Error as PandemiaError, error::ErrorCode};

use failure;
use std::io;

/// List of possible API errors.
#[derive(Fail, Debug)]
pub enum Error {
    /// Input/output error. This type includes errors related to files that are not
    /// a part of the Exonum storage.
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),

    /// Bad request. This error occurs when the request contains invalid syntax.
    #[fail(display = "Bad request: {}", _1)]
    BadRequest(i32, String),

    /// Not found. This error occurs when the server cannot locate the requested
    /// resource.
    #[fail(display = "Not found: {}", _1)]
    NotFound(i32, String),

    /// Internal server error. This type can return any internal server error to the user.
    #[fail(display = "Internal server error: {}", _1)]
    InternalError(i32, #[cause] failure::Error),

    /// Error yang muncul apabila user menginputkan parameter yang tidak sesuai
    #[fail(display = "Invalid parameter: {}", _1)]
    InvalidParameter(i32, String),

    /// Error yang muncul ketika sebuah object unik telah ada
    /// biasanya dimunculkan oleh operasi creation.
    #[fail(display = "Already exists")]
    AlreadyExists,

    /// Error yang muncul ketika suatu object telah habis masa berlakunya
    /// pada saat transaksi misalnya.
    #[fail(display = "{} expired", _0)]
    Expired(&'static str),

    /// Error yang bisa digunakan untuk menampilkan kode dan deskripsi secara custom.
    #[fail(display = "error code {}: {}", _0, _1)]
    CustomError(i32, String),

    /// Unauthorized error. This error occurs when the request lacks valid
    /// authentication credentials.
    #[fail(display = "Unauthorized")]
    Unauthorized,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Self {
        Error::InternalError(ErrorCode::UnknownError as i32, e)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::BadRequest(
            ErrorCode::SerializeDeserializeError as i32,
            "Invalid data".to_string(),
        )
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(e: validator::ValidationErrors) -> Self {
        // dbg!(&e);
        for (k, v) in e.errors() {
            // dbg!(&v);
            match v {
                ValidationErrorsKind::Field(fields) => {
                    let field = fields.last().expect("Empty error fields");
                    let param_value = field
                        .params
                        .get("value")
                        .map(|a| format!("{}", a))
                        .unwrap_or("".to_string());
                    let message = match &field.message {
                        Some(msg) => msg.to_string().replace("{}", &param_value),
                        _ => format!("Invalid {}: {}", field.code, param_value),
                    };
                    return Error::InvalidParameter(ErrorCode::InvalidParameter as i32, message);
                }
                _ => (),
            }
        }
        Error::InvalidParameter(ErrorCode::InvalidParameter as i32, "Invalid parameter".to_owned())
    }
}

use diesel::result::DatabaseErrorKind;

impl From<PandemiaError> for Error {
    fn from(e: PandemiaError) -> Self {
        match &e {
            PandemiaError::Storage(diesel::result::Error::DatabaseError(kind, msg)) => {
                error!("error: {:?}", &msg);
                match kind {
                    DatabaseErrorKind::UniqueViolation | DatabaseErrorKind::ForeignKeyViolation => {
                        Error::AlreadyExists
                    }
                    _ => Error::CustomError(ErrorCode::DatabaseError as i32, "Internal error".to_owned()),
                }
            }
            PandemiaError::Storage(diesel::result::Error::NotFound) => Error::NotFound(
                ErrorCode::DatabaseRecordNotFoundError as i32,
                "Not found".to_owned(),
            ),
            PandemiaError::Unauthorized => Error::Unauthorized,
            PandemiaError::InvalidParameter(msg) => {
                Error::InvalidParameter(ErrorCode::InvalidParameter as i32, e.to_string())
            }
            PandemiaError::BadRequest(code, msg) => Error::BadRequest(*code, e.to_string()),
            _ => Error::InternalError(ErrorCode::DatabaseError as i32, failure::Error::from(e)),
        }
    }
}

use actix_web::{HttpResponse, ResponseError};

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::BadRequest(code, err) => HttpResponse::Ok().json(ApiResult::error(*code, err.to_owned())),
            Error::InternalError(code, err) => {
                HttpResponse::Ok().json(ApiResult::error(*code, err.to_string()))
            }
            Error::Io(err) => HttpResponse::InternalServerError()
                .json(ApiResult::error(ErrorCode::UnknownError as i32, err.to_string())),
            Error::NotFound(code, err) => HttpResponse::Ok().json(ApiResult::error(*code, err.to_string())),
            Error::InvalidParameter(code, d) => {
                HttpResponse::Ok().json(ApiResult::error(*code, d.to_owned()))
            }
            Error::AlreadyExists => {
                HttpResponse::Ok().json(ApiResult::error(304, "Already exists".to_owned()))
            }
            Error::CustomError(code, d) => HttpResponse::Ok().json(ApiResult::error(*code, d.to_owned())),
            Error::Unauthorized => {
                // HttpResponse::Unauthorized().finish()
                HttpResponse::Ok().json(ApiResult::error(
                    ErrorCode::Unauthorized as i32,
                    "Unauthorized".to_owned(),
                ))
            }
            Error::Expired(d) => HttpResponse::Ok().json(ApiResult::error(4001, format!("{}", self))),
        }
    }
}

/// Build parameter error
pub fn param_error<T>(msg: &str) -> Result<T, Error> {
    Err(Error::InvalidParameter(
        ErrorCode::InvalidParameter as i32,
        msg.to_string(),
    ))?;
    panic!("Unhandled error: {}", msg);
}

/// Build parameter error
pub fn unauthorized<T>() -> Result<T, Error> {
    Err(Error::Unauthorized)?;
    panic!("Unhandled error");
}
