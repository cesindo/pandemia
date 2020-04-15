use diesel::{sql_query, sql_types};

sql_function!(
    /// To lowerize sql column value typely
    fn lower(x: sql_types::Text) -> sql_types::Text);

sql_function!(
    /// To append array item in Postgres
    fn array_append<T>(list: sql_types::Array<T>, item: T) -> sql_types::Array<sql_types::Text>
);

sql_function!(
    /// To remove array item in Postgres
    fn array_remove<T>(list: sql_types::Array<T>, item: T) -> sql_types::Array<sql_types::Text>
);

sql_function!(
    /// To remove array item in Postgres
    fn array_replace<T1, T2>(list: sql_types::Array<T1>, item1: T1, item2: T2) -> sql_types::Array<sql_types::Text>
);

sql_function!(
    /// Extends array with other array in Postgres
    fn array_cat<T>(list: sql_types::Array<T>, item: sql_types::Array<T>) -> sql_types::Array<T>
);
