table! {
    access_tokens (token) {
        token -> Text,
        user_id -> Int8,
        created -> Timestamp,
        valid_thru -> Timestamp,
    }
}

table! {
    addresses (id) {
        id -> Int8,
        user_id -> Int8,
        kind -> Int4,
        address -> Text,
        regency -> Varchar,
        province -> Varchar,
        country -> Varchar,
        phone_num -> Varchar,
        active -> Bool,
        notes -> Text,
    }
}

table! {
    admin_access_tokens (token) {
        token -> Text,
        admin_id -> Int8,
        created -> Timestamp,
        valid_thru -> Timestamp,
    }
}

table! {
    admin_passhash (id) {
        id -> Int8,
        admin_id -> Int8,
        passhash -> Varchar,
        deprecated -> Bool,
        ver -> Int4,
        created -> Timestamp,
    }
}

table! {
    admins (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        phone_num -> Varchar,
        labels -> Array<Text>,
        active -> Bool,
        register_time -> Timestamp,
    }
}

table! {
    feeds (id) {
        id -> Int8,
        creator_id -> Int8,
        creator_name -> Varchar,
        loc -> Text,
        kind -> Int2,
        text -> Text,
        hashtags -> Array<Text>,
        meta -> Array<Text>,
        ts -> Timestamp,
    }
}

table! {
    notifs (id) {
        id -> Int8,
        kind -> Int2,
        text -> Varchar,
        initiator_id -> Int8,
        receiver_id -> Int8,
        read -> Bool,
        keywords -> Array<Text>,
        meta -> Array<Text>,
        ts -> Timestamp,
    }
}

table! {
    records (id) {
        id -> Int8,
        loc -> Text,
        loc_kind -> Int2,
        total_cases -> Int4,
        total_deaths -> Int4,
        total_recovered -> Int4,
        active_cases -> Int4,
        critical_cases -> Int4,
        cases_to_pop -> Float8,
        meta -> Array<Text>,
        last_updated -> Timestamp,
    }
}

table! {
    register_users (token) {
        token -> Varchar,
        full_name -> Varchar,
        email -> Varchar,
        phone_num -> Varchar,
        register_time -> Timestamp,
        code -> Varchar,
    }
}

table! {
    reset_password_admins (admin_id) {
        admin_id -> Int8,
        token -> Varchar,
        created -> Timestamp,
        expiration -> Nullable<Timestamp>,
    }
}

table! {
    user_connect (device_id) {
        device_id -> Varchar,
        provider_name -> Varchar,
        app_id -> Varchar,
    }
}

table! {
    user_keys (id) {
        id -> Int8,
        user_id -> Int8,
        pub_key -> Text,
        secret_key -> Text,
        created -> Timestamp,
        active -> Bool,
    }
}

table! {
    user_passhash (user_id) {
        user_id -> Int8,
        passhash -> Varchar,
        deprecated -> Bool,
        ver -> Int4,
        created -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        full_name -> Varchar,
        email -> Varchar,
        phone_num -> Varchar,
        active -> Bool,
        register_time -> Timestamp,
    }
}

joinable!(access_tokens -> users (user_id));
joinable!(addresses -> users (user_id));
joinable!(admin_access_tokens -> admins (admin_id));
joinable!(admin_passhash -> admins (admin_id));
joinable!(feeds -> users (creator_id));
joinable!(notifs -> users (receiver_id));
joinable!(reset_password_admins -> admins (admin_id));
joinable!(user_keys -> users (user_id));
joinable!(user_passhash -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    addresses,
    admin_access_tokens,
    admin_passhash,
    admins,
    feeds,
    notifs,
    records,
    register_users,
    reset_password_admins,
    user_connect,
    user_keys,
    user_passhash,
    users,
);
