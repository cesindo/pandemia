//! Koleksi macro library internal

macro_rules! implement_crypto_wrapper {
    ( $(#[$attr:meta])*  struct $name:ident, $size:expr) => {
        implement_crypto_wrapper!( $(#[$attr])* struct $name, $crate::crypto::ds::$name, $name, $size );
    };
    ( $(#[$attr:meta])* struct $name:ident, $source:path, $source_name:ident, $size:expr) => {
        /// Crypto object wrapper
        #[derive(Clone)]
        $(#[$attr])*
        pub struct $name([u8; $size]);

        impl $name {
            #[doc(hidden)]
            pub fn new(bytes_array: [u8; $size]) -> Self {
                let a = {
                    use $source;
                    $source_name::from_bytes(&bytes_array).expect("from bytes")
                };
                $name(a.to_bytes())
            }

            /// Creates new instance from bytes slice.
            #[inline]
            pub fn from_slice(bytes: &[u8]) -> Option<Self> {
                // kode ini kelihatan aneh, tapi hanya dengan cara inilah
                // kode bagian ini bisa dicompile di Rust stable.
                // kemungkinan kalau nanti Rust stable sudah bisa menghandle
                // macro type path agar bisa langsung digunakan untuk memanggil
                // fungsi statis-nya kode ini akan dirubah.

                let a = {
                    use $source;
                    $source_name::from_bytes(bytes)
                };
                a.map(|a| $name(a.to_bytes())).ok()
            }

            /// Convert to hex string
            #[inline]
            pub fn to_hex(&self) -> String {
                hex::encode(&self.0[..])
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({}..)", stringify!($name), &self.to_hex()[..8])
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({}..)", stringify!($name), &self.to_hex()[..8])
            }
        }

        impl ::hex::FromHex for $name {
            type Error = ::hex::FromHexError;

            fn from_hex<T: AsRef<[u8]>>(v: T) -> Result<Self, Self::Error> {
                let bytes = Vec::<u8>::from_hex(v)?;
                if let Some(self_value) = Self::from_slice(bytes.as_ref()) {
                    Ok(self_value)
                } else {
                    Err(::hex::FromHexError::InvalidStringLength)
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::hex::FromHexError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use hex::FromHex;
                $name::from_hex(s)
            }
        }
    };
}

macro_rules! impl_event_listener {
    ($name:ident) => {
        impl $name {
            pub fn new() -> Arc<Self> {
                Arc::new(Self { db: db::clone() })
            }

            fn db(&self) -> db::DbConn {
                self.db.get().expect(concat!(
                    "Cannot get db connection from poll in ",
                    stringify!($name)
                ))
            }
        }
    };
}

macro_rules! impl_dao {
    ( $(#[$meta:meta])* $name:ident) => {

        $(#[$meta])*
        #[derive(Dao)]
        pub struct $name<'a> {
            db: &'a PgConnection,
        }
    };
    ( $(#[$meta:meta])* $name:ident, $id_type:literal) => {

        $(#[$meta])*
        #[derive(Dao)]
        #[id_type = $id_type]
        pub struct $name<'a> {
            db: &'a PgConnection,
        }
    };
}

/// Macro to generate Dao implementation.
///
/// Example use of this macro:
///
/// ```
/// impl_daos!(
///     /// DAO for Comment
///     CommentDao
/// );
/// ```
///
/// For custom ID type
///
/// ```
/// impl_daos!(
///     /// DAO for Role
///     (RoleDao, "i32"),
/// );
/// ```
macro_rules! impl_daos {
    ( $( $(#[$meta:meta])* $name:ident, )* ) => {
        $( impl_dao!(
            $(#[$meta])*
            $name
            );
        )*
    };
    ( $( $(#[$meta:meta])* $name:ident ),* ) => {
        impl_daos!( $( $(#[$meta])* $name, )* );
    };
    ( $( $(#[$meta:meta])* ( $name:ident, $id_type:literal) ,)* ) => {
        $( impl_dao!(
            $(#[$meta])*
            $name, $id_type
            );
        )*
    };
    ( $( $(#[$meta:meta])* ( $name:ident, $id_type:literal)),* ) => {
        impl_daos!(
            $( $(#[$meta])* ( $name, $id_type), )*
        );
    };
}

macro_rules! meta_value_i32 {
    ($s:ident, $key:literal) => {
        $s.meta
            .iter()
            .find(|a| a.starts_with(concat!($key, ":")))
            .and_then(|a| a.splitn(2, ':').last())
            .and_then(|a| a.parse::<i32>().ok())
            .unwrap_or(0)
    };
}

macro_rules! meta_value_str {
    ($s:ident, $key:literal) => {
        $s.meta
            .iter()
            .find(|a| a.starts_with(concat!($key, ":")))
            .and_then(|a| a.splitn(2, ':').last())
            .unwrap_or("")
    };
}
