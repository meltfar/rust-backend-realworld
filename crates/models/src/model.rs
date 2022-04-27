#[derive(Queryable)]
pub struct Module {
    pub id: i64,
    pub module_name: String,
    pub module_desc: String,
    pub global_config_names: String,
    pub worker_config_names: String,
    pub request_config_names: String,
    pub version: i32,
    pub full_name: String,
    pub sort_index: i32,
}

#[derive(Queryable, Debug)]
// #[table_name = "config_version"]
pub struct ConfigVersion {
    pub id: i32,
    pub version: i32,
    pub sub_version: String,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}
