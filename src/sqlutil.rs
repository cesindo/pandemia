use diesel::{sql_query, sql_types};

sql_function!(
    /// To lowerize sql column value typely
    fn lower(x: sql_types::Text) -> sql_types::Text);
