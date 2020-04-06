//! Definisi error untuk system internal

use diesel;
use failure;
use std::io;

/// Listing dari jenis error yang mungkin muncul pada sistem internal
#[derive(Fail, Debug)]
pub enum Error {
    /// Storage error. This type includes errors related to the database, caused
    /// by, for example, serialization issues.
    #[fail(display = "Storage error: {}", _0)]
    Storage(#[cause] diesel::result::Error),

    /// Input/output error. This type includes errors related to files that are not
    /// a part of the Exonum storage.
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),

    /// Bad request. This error occurs when the request contains invalid syntax.
    #[fail(display = "Bad request: {}", _1)]
    BadRequest(i32, String),

    /// Not found. This error occurs when the server cannot locate the requested
    /// resource.
    #[fail(display = "Not found: {}", _0)]
    NotFound(String),

    /// Internal server error. This type can return any internal server error to the user.
    #[fail(display = "Internal server error: {}", _0)]
    InternalError(failure::Error),

    /// Error yang muncul apabila user menginputkan parameter yang tidak sesuai
    #[fail(display = "Invalid parameter: {}", _0)]
    InvalidParameter(String),

    /// Error yang muncul ketika sebuah object unik telah ada
    /// biasanya dimunculkan oleh operasi creation.
    #[fail(display = "Already exists")]
    AlreadyExists,

    /// Error yang muncul apabila sesuatu tidak mencukupi seperti saldo misalnya.
    #[fail(display = "{}", _0)]
    Insufficient(&'static str),

    /// Error yang bisa digunakan untuk menampilkan kode dan deskripsi secara custom.
    #[fail(display = "error code {}: {}", _1, _0)]
    CustomError(String, i32),

    /// Unauthorized error. This error occurs when the request lacks valid
    /// authentication credentials.
    #[fail(display = "Unauthorized")]
    Unauthorized,
}

/// Definisi kode kesalahan
pub enum ErrorCode {
    /// Sukses atau tidak terjadi error.
    NoError = 0,
    /// Unauthorized
    Unauthorized = 3000,

    /// Kegagalan yang berkaitan dengan proses serialize/deserialize data.
    SerializeDeserializeError = 4001,
    /// Parameter tidak lengkap/kurang.
    InvalidParameter = 4002,
    /// Message tidak ada signature-nya, dibutuhkan untuk verifikasi menggunakan public key.
    MessageHasNoSign = 4003,
    /// Tidak ada informasi login.
    NoLoginInfo = 4004,
    /// Pengirim dan penerima alamatnya sama.
    FromAndToTargetIsSame = 4005,

    /// Kegagalan yang tidak diketahui penyebabnya.
    UnknownError = 5001,

    /// Kegagalan pada database internal apabila terjadi error.
    DatabaseError = 6001,
    /// Kegagalan pada database yang berkaitan dengan
    /// ketidakditemukannya record/data di dalam database.
    DatabaseRecordNotFoundError = 6002,
    // Tambahkan definisi kode error mu sendiri di sini.
}

// semua error yang berasal dari diesel akan dipropagasi ke sistem error [Error::Storage]
impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::Storage(e)
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

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::CustomError("Http requewst to third party failed".to_string(), 500)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        error!("json serialize/deserialize error: {}", e);
        Error::BadRequest(
            ErrorCode::SerializeDeserializeError as i32,
            "Invalid data".to_string(),
        )
    }
}
