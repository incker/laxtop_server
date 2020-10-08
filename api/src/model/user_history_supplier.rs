use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

pub struct UserHistorySupplier {
    user_id: u32,
    supplier_id: u32,
}

impl UserHistorySupplier {
    pub fn get_user_history_supplier_ids(user_id: u32, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::user_history_supplier::{self, dsl};
        user_history_supplier::table
            .select(dsl::supplier_id)
            .filter(dsl::user_id.eq(user_id))
            .load(conn)
            .expect("Some fail here: UserSpot::select_by_user :(")
    }
}
