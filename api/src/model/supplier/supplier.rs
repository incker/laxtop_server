use chrono::{NaiveDate, NaiveDateTime};
use diesel::{sql_query, update, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::{Location, Session, SupplierStatus};
use crate::schema::supplier;

#[derive(Debug, Queryable, PartialEq)]
pub struct Supplier {
    pub id: u32,
    pub login: String,
    pub password: String,
    pub name: String,
    pub about: String,
    pub address: String,
    pub location: Location,
    pub status: SupplierStatus,
    pub chat_id: i64,
    pub shift: u8,
}

impl Supplier {
    pub fn add_session(&self, conn: &MysqlConnection) -> Session {
        Session::new_session_supplier(self.id, conn)
    }

    pub fn get_by_chat_id(chat_id: i64, conn: &MysqlConnection) -> Option<u32> {
        use crate::schema::supplier::dsl;

        supplier::table
            .select(dsl::id)
            .filter(dsl::chat_id.eq(chat_id))
            .first::<u32>(conn)
            .ok()
    }

    pub fn get_chat_id(supplier_id: u32, conn: &MysqlConnection) -> Option<i64> {
        use crate::schema::supplier::dsl;

        let res = supplier::table
            .select(dsl::chat_id)
            .filter(dsl::id.eq(supplier_id))
            .first::<i64>(conn)
            .ok();

        if res == Some(0) {
            None
        } else {
            res
        }
    }

    pub fn set_chat_id(
        supplier_id: u32,
        chat_id: i64,
        telegram_user_id: i64,
        conn: &MysqlConnection,
    ) {
        use crate::schema::supplier::dsl;
        let target = supplier::table.filter(dsl::id.eq(supplier_id));
        update(target)
            .set((
                dsl::chat_id.eq(chat_id),
                dsl::telegram_user_id.eq(telegram_user_id),
                dsl::status.eq(SupplierStatus::Active),
            ))
            .execute(conn)
            .unwrap();
    }

    pub fn exists(supplier_id: u32, conn: &MysqlConnection) -> Result<(), (String, String)> {
        use crate::schema::supplier::dsl;

        let exists: bool = supplier::table
            .select(dsl::id)
            .filter(dsl::id.eq(supplier_id))
            .first::<u32>(conn)
            .is_ok();

        if exists {
            Ok(())
        } else {
            Err((
                "supplierId".into(),
                format!("supplier id '{}'  not exist", supplier_id),
            ))
        }
    }

    // make shift (update number) when supplier info or catalog was updated
    pub fn increment_supplier_shift(supplier_id: u32, conn: &MysqlConnection) {
        let query = format!("UPDATE `supplier` SET `shift` = IF(`shift` = 255, 0, `shift` + 1) WHERE `supplier`.`id` = {} LIMIT 1;", supplier_id);
        let _users = sql_query(&query).execute(conn).unwrap();
    }

    pub fn get_supplier_id(telegram_login: i64, conn: &MysqlConnection) -> Option<u32> {
        use crate::schema::supplier::dsl;
        supplier::table
            .select(dsl::id)
            .filter(dsl::telegram_login.eq(telegram_login))
            .first::<u32>(conn)
            .ok()
    }
}
