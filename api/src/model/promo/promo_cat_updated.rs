use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};
use diesel::{
    sql_query, update, Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::{Location, Promo, Session, SupplierStatus, ANNUAL_PROMO_AMOUNT};

pub struct PromoCatUpdated(NaiveDateTime);

impl PromoCatUpdated {
    pub fn select(conn: &MysqlConnection) -> Self {
        use crate::schema::promo_cat_updated::{self, dsl};

        let date = promo_cat_updated::table
            .select(dsl::updated_at)
            .first::<NaiveDateTime>(conn)
            .unwrap();
        PromoCatUpdated(date)
    }

    pub fn update(conn: &MysqlConnection) {
        use crate::schema::promo_cat_updated::{self, dsl};

        update(promo_cat_updated::table)
            .set(dsl::updated_at.eq(diesel::dsl::now))
            .execute(conn)
            .unwrap();
    }
}
