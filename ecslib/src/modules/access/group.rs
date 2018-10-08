use crate::schema::access_groups;
#[derive(Insertable, AsChangeset, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
#[table_name = "access_groups"]
pub struct AccessGroup {
    pub id: i64,
    pub name: String,
    pub access_control_id: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessGroupData {
    pub name: String,
}
