use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::schema::user_spot;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "user_spot"]
pub struct UserSpot {
    pub user_id: u32,
    pub spot_id: u32,
}

impl UserSpot {
    pub fn select_by_user(user_id: u32, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::user_spot::dsl;

        user_spot::table
            .select(dsl::spot_id)
            .filter(dsl::user_id.eq(user_id))
            .load(conn)
            .expect("Some fail here: UserSpot::select_by_user :(")
    }

    pub fn insert_values(user_id: u32, spot_id: u32, conn: &MysqlConnection) {
        use crate::schema::user_spot::dsl;
        let _res = diesel::insert_into(user_spot::table)
            .values((dsl::user_id.eq(user_id), dsl::spot_id.eq(spot_id)))
            .execute(conn);
    }

    pub fn validate_ligament(
        user_id: u32,
        spot_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), (String, String)> {
        use crate::schema::user_spot::dsl;
        user_spot::table
            .select(dsl::spot_id)
            .filter(dsl::user_id.eq(user_id))
            .first::<u32>(conn)
            .map(|_| ())
            .map_err(|_| {
                (
                    "user_spot".to_string(),
                    format!("user_id {} does not own spot_id {}", user_id, spot_id),
                )
            })
    }
}
