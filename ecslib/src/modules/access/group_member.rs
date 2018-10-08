use crate::schema::access_group_members;
#[derive(Insertable, AsChangeset, Queryable, Associations, Serialize, Deserialize, Debug, Clone)]
#[table_name = "access_group_members"]
pub struct AccessGroupMember {
    pub id: i64,
    pub access_group_id: i64,
    pub user_id: i64,
    pub access_control_id: i64,
}
