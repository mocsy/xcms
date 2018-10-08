use crate::schema::access_keys;
use chrono::{DateTime, Utc};
#[derive(Insertable, AsChangeset, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
#[table_name = "access_keys"]
pub struct AccessKey {
    pub id: i64,
    pub key: String,
    pub access_type: String,
    pub user_id: i64,
    pub reason: String,
    pub expiry: DateTime<Utc>,
    pub access_control_id: i64,
}
