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
        meta -> Array<Text>,
        active -> Bool,
        register_time -> Timestamp,
    }
}

table! {
    cities (id) {
        id -> Int8,
        name -> Text,
        province -> Text,
        country_code -> Text,
        area_code -> Varchar,
        ts -> Timestamp,
    }
}

table! {
    district_data (id) {
        id -> Int8,
        district_id -> Int8,
        odp -> Int4,
        pdp -> Int4,
        cases -> Int4,
        recovered -> Int4,
        deaths -> Int4,
        last_updated -> Timestamp,
        last_updated_by_id -> Int8,
        city_id -> Int8,
        meta -> Array<Text>,
        ts -> Timestamp,
    }
}

table! {
    districts (id) {
        id -> Int8,
        name -> Text,
        city_id -> Int8,
        meta -> Array<Text>,
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
    geoloc_cache (id) {
        id -> Int8,
        name -> Text,
        latitude -> Float8,
        longitude -> Float8,
        ts -> Timestamp,
    }
}

table! {
    kv_store (id) {
        id -> Int8,
        a_key -> Text,
        a_val -> Text,
    }
}

table! {
    logs (id) {
        id -> Int8,
        activity -> Text,
        initiator_id -> Int8,
        meta -> Array<Text>,
        ts -> Timestamp,
    }
}

table! {
    map_markers (id) {
        id -> Int8,
        name -> Text,
        info -> Text,
        latitude -> Float8,
        longitude -> Float8,
        kind -> Int2,
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
        latest -> Bool,
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
    report_notes (id) {
        id -> Int8,
        title -> Text,
        notes -> Text,
        creator_id -> Int8,
        creator_name -> Text,
        city_id -> Int8,
        published -> Bool,
        meta -> Array<Text>,
        ts -> Timestamp,
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
    sub_reports (id) {
        id -> Int8,
        creator_id -> Int8,
        creator_name -> Varchar,
        full_name -> Varchar,
        age -> Int4,
        residence_address -> Varchar,
        gender -> Varchar,
        coming_from -> Varchar,
        arrival_date -> Date,
        healthy -> Int4,
        notes -> Varchar,
        status -> Int4,
        meta -> Array<Text>,
        ts -> Timestamp,
        city_id -> Int8,
        district_id -> Int8,
        village_id -> Int8,
    }
}

table! {
    user_connect (device_id) {
        device_id -> Text,
        user_id -> Int8,
        provider_name -> Varchar,
        app_id -> Text,
        enable_push_notif -> Bool,
        latest_loc -> Text,
        latest_loc_full -> Text,
        latest_loc_long -> Float8,
        latest_loc_lat -> Float8,
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
    user_settings (id) {
        id -> Int8,
        user_id -> Int8,
        s_key -> Text,
        s_value -> Text,
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
        latitude -> Float8,
        longitude -> Float8,
        meta -> Array<Text>,
    }
}

table! {
    village_data (id) {
        id -> Int8,
        village_id -> Int8,
        odp -> Int4,
        pdp -> Int4,
        cases -> Int4,
        recovered -> Int4,
        deaths -> Int4,
        last_updated -> Timestamp,
        last_updated_by_id -> Int8,
        ts -> Timestamp,
        city_id -> Int8,
        meta -> Array<Text>,
    }
}

table! {
    villages (id) {
        id -> Int8,
        name -> Text,
        district_name -> Text,
        city -> Text,
        province -> Text,
        latitude -> Float8,
        longitude -> Float8,
        meta -> Array<Text>,
        ts -> Timestamp,
        city_id -> Int8,
        district_id -> Int8,
    }
}

joinable!(access_tokens -> users (user_id));
joinable!(addresses -> users (user_id));
joinable!(admin_access_tokens -> admins (admin_id));
joinable!(admin_passhash -> admins (admin_id));
joinable!(district_data -> districts (district_id));
joinable!(districts -> cities (city_id));
joinable!(feeds -> users (creator_id));
joinable!(logs -> users (initiator_id));
joinable!(notifs -> users (receiver_id));
joinable!(report_notes -> cities (city_id));
joinable!(report_notes -> users (creator_id));
joinable!(reset_password_admins -> admins (admin_id));
joinable!(sub_reports -> cities (city_id));
joinable!(sub_reports -> users (creator_id));
joinable!(user_connect -> users (user_id));
joinable!(user_keys -> users (user_id));
joinable!(user_passhash -> users (user_id));
joinable!(user_settings -> users (user_id));
joinable!(village_data -> cities (city_id));
joinable!(village_data -> users (last_updated_by_id));
joinable!(village_data -> villages (village_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    addresses,
    admin_access_tokens,
    admin_passhash,
    admins,
    cities,
    district_data,
    districts,
    feeds,
    geoloc_cache,
    kv_store,
    logs,
    map_markers,
    notifs,
    records,
    register_users,
    report_notes,
    reset_password_admins,
    sub_reports,
    user_connect,
    user_keys,
    user_passhash,
    user_settings,
    users,
    village_data,
    villages,
);
