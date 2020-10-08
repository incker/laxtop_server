use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

pub struct UserHistoryProduct {
    user_id: u32,
    product_id: u32,
}

impl UserHistoryProduct {
    pub fn get_user_history_product_ids(user_id: u32, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::user_history_product::{self, dsl};
        user_history_product::table
            .select(dsl::product_id)
            .filter(dsl::user_id.eq(user_id))
            .load(conn)
            .expect("Some fail here: UserSpot::select_by_user :(")
    }
}
