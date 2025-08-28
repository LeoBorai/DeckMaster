pub trait ToSqlQuery: Clone + Send + Sync {
    fn to_sql_query(&self) -> String;
}
