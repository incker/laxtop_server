use chrono::NaiveDateTime;
use diesel::{
    update, BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::OwnerType;
use crate::schema::user;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: u32,
    pub number: String,
    pub country_code: String,
    pub name: String,
    pub status: u8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn create_if_not_exist(
        number: &str,
        name: &str,
        creator_type: OwnerType,
        creator_id: u32,
        conn: &MysqlConnection,
    ) -> u32 {
        use crate::schema::user::dsl;
        let user_id = User::select_id_by_number(number, conn).unwrap_or({
            let _res = diesel::insert_into(user::table)
                .values((
                    dsl::number.eq(number),
                    dsl::name.eq(name),
                    dsl::creator_type.eq(creator_type),
                    dsl::creator_id.eq(creator_id),
                ))
                .execute(conn);
            User::select_id_by_number(number, conn).unwrap()
        });
        // need optimize? or even delete?
        if creator_id == 0 && creator_type == OwnerType::User {
            update(user::table.filter(dsl::id.eq(user_id).and(dsl::creator_id.eq(0))))
                .set(dsl::creator_id.eq(user_id))
                .execute(conn)
                .unwrap();
        }
        user_id
    }

    pub fn select_id_by_number(number: &str, conn: &MysqlConnection) -> Option<u32> {
        use crate::schema::user::dsl;
        user::table
            .select(dsl::id)
            .filter(dsl::number.eq(number))
            .first(conn)
            .ok()
    }
}
