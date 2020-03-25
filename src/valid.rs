use chrono::Utc;

use crate::models::{AccessToken, AdminAccessToken};

/// Trait untuk memastikan apakah suatu object
/// bisa divalidasi atau tidak.
pub trait Validable {
    /// Periksa kevalidan object.
    fn valid(&self) -> bool;
}

impl Validable for AccessToken {
    fn valid(&self) -> bool {
        let now = Utc::now().naive_utc();
        now < self.valid_thru
    }
}

/// Trait untuk memastikan apakah suatu object
/// bisa expired atau tidak.
pub trait Expirable {
    /// Periksa apakah object sudah expired.
    fn expired(&self) -> bool;
}

impl Expirable for AccessToken {
    fn expired(&self) -> bool {
        let now = Utc::now().naive_utc();
        now > self.valid_thru
    }
}

impl Expirable for AdminAccessToken {
    fn expired(&self) -> bool {
        let now = Utc::now().naive_utc();
        now > self.valid_thru
    }
}

#[cfg(test)]
mod tests {
    use super::{Expirable, Validable};
    use crate::models::AccessToken;
    use chrono::{Duration, Utc};
    use std::{ops::Add, thread::sleep, time};

    #[test]
    fn test_access_token_valid() {
        let access_token = AccessToken {
            token: "".to_owned(),
            user_id: 1,
            created: Utc::now().naive_utc(),
            valid_thru: Utc::now().naive_utc().add(Duration::days(1)),
        };
        sleep(time::Duration::from_millis(1000));
        assert!(access_token.valid());
    }

    #[test]
    fn test_access_token_expire() {
        let access_token = AccessToken {
            token: "".to_owned(),
            user_id: 1,
            created: Utc::now().naive_utc(),
            valid_thru: Utc::now().naive_utc().add(Duration::milliseconds(50)),
        };
        sleep(time::Duration::from_millis(1000));
        assert!(!access_token.valid());
        assert!(access_token.expired());
    }
}
