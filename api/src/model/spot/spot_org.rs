use std::collections::HashMap;

use diesel::{
    delete, update, BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl,
    RunQueryDsl,
};
use rocket_contrib::json::Json;

use crate::base::{RespErrors, ValidateFormatter};
use crate::schema::spot_org;

#[derive(Debug, Queryable, PartialEq, Serialize, Deserialize, Validate, Default)]
pub struct SpotOrg {
    #[validate(length(min = 1, max = 255, message = "Can not be empty"))]
    #[serde(rename = "orgType")]
    pub org_type: String,
    #[validate(length(min = 1, max = 255, message = "Can not be empty"))]
    #[serde(rename = "orgName")]
    pub org_name: String,
}

impl SpotOrg {
    pub fn insert_or_update(&self, user_id: u32, spot_id: u32, conn: &MysqlConnection) {
        use crate::schema::spot_org::dsl;

        let insert_res = diesel::insert_into(spot_org::table)
            .values((
                dsl::user_id.eq(user_id),
                dsl::spot_id.eq(spot_id),
                dsl::org_type.eq(&self.org_type),
                dsl::org_name.eq(&self.org_name),
            ))
            .execute(conn);
        if insert_res.is_err() {
            self.update(user_id, spot_id, conn);
        }
    }

    pub fn create_ligament(
        user_id: u32,
        spot_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::spot_org::dsl;

        diesel::insert_into(spot_org::table)
            .values((
                dsl::user_id.eq(user_id),
                dsl::spot_id.eq(spot_id),
                dsl::org_type.eq(""),
                dsl::org_name.eq(""),
            ))
            .execute(conn)
            .map(|_| ())
    }

    pub fn select(user_id: u32, spot_id: u32, conn: &MysqlConnection) -> Self {
        use crate::schema::spot_org::dsl;

        spot_org::table
            .select((dsl::org_type, dsl::org_name))
            .filter(dsl::user_id.eq(user_id).and(dsl::spot_id.eq(spot_id)))
            .first(conn)
            .unwrap_or_default()
    }

    pub fn select_by_ids(
        user_id: u32,
        spot_ids: &[u32],
        conn: &MysqlConnection,
    ) -> HashMap<u32, Self> {
        use crate::schema::spot_org::dsl;

        let mut hash_map = HashMap::with_capacity(spot_ids.len());

        let rows: Vec<(u32, String, String)> = spot_org::table
            .select((dsl::spot_id, dsl::org_type, dsl::org_name))
            .filter(dsl::user_id.eq(user_id).and(dsl::spot_id.eq_any(spot_ids)))
            .load(conn)
            .expect("select_by_user_and_spot_ids");

        for (spot_id, org_type, org_name) in rows {
            hash_map.insert(spot_id, SpotOrg { org_type, org_name });
        }

        hash_map
    }

    pub fn update(&self, user_id: u32, spot_id: u32, conn: &MysqlConnection) {
        use crate::schema::spot_org::dsl;

        let target = spot_org::table.filter(dsl::user_id.eq(user_id).and(dsl::spot_id.eq(spot_id)));

        update(target)
            .set((
                dsl::org_type.eq(&self.org_type),
                dsl::org_name.eq(&self.org_name),
            ))
            .execute(conn)
            .unwrap();
    }

    pub fn delete(user_id: u32, spot_id: u32, conn: &MysqlConnection) {
        use crate::schema::spot_org::dsl;
        let target = dsl::user_id.eq(user_id).and(dsl::spot_id.eq(spot_id));
        let _res = delete(spot_org::table.filter(target)).execute(conn);
    }

    pub fn rocket_validate(&self) -> Result<(), Json<RespErrors>> {
        self.run_validator().map_err(Json)
    }
}
