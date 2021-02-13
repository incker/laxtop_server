use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};
use diesel::{
    sql_query, update, Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::{Location, Promo, Session, SupplierStatus, ANNUAL_PROMO_AMOUNT};
use crate::schema::supplier;

/*
1. select all suppliers registered this year,
2. depends on registered month subtract available promos
3. by adding dummy promos in promo table
*/

pub fn shrink_new_suppliers_promo_amount(conn: &MysqlConnection) {
    let rest_supplier_promos = get_suppliers_promo_amount_to_skip(conn);
    // transaction not to shrink suppliers twice
    conn.transaction::<(), diesel::result::Error, _>(|| {
        for (supplier_id, dummy_promo_amount) in rest_supplier_promos {
            let current_promo_amount = Promo::count_annual_supplier_promos(supplier_id, conn);
            for _ in current_promo_amount..dummy_promo_amount {
                Promo::insert_dummy_promo(supplier_id, conn);
            }
        }
        Ok(())
    });
}

fn get_suppliers_promo_amount_to_skip(conn: &MysqlConnection) -> Vec<(u32, i32)> {
    let year = current_year();
    let supplier_creations = select_suppliers_by_created_at(year, conn);

    supplier_creations
        .into_iter()
        .map(|(supplier_id, created_at)| {
            (
                supplier_id,
                count_amount_of_promos_to_skip(ANNUAL_PROMO_AMOUNT, created_at.month() as i32),
            )
        })
        .collect()
}

/// Get suppliers that was registered in a year, to shrink amount of available promos
fn select_suppliers_by_created_at(year: i32, conn: &MysqlConnection) -> Vec<(u32, NaiveDateTime)> {
    use crate::schema::supplier::dsl;
    // month is 2 cause no need to shrink promo amount for suppliers registered in january
    let date = NaiveDate::from_ymd(year, 2, 1).and_hms(0, 0, 0);

    supplier::table
        .select((dsl::id, dsl::created_at))
        .filter(dsl::created_at.ge(date))
        .load::<(u32, NaiveDateTime)>(conn)
        .unwrap()
}

fn current_year() -> i32 {
    let (_is_common_era, year) = Utc::now().year_ce();
    year as i32
}

/// shrink total amount depends how many month was passed
fn count_amount_of_promos_to_skip(total: i32, month: i32) -> i32 {
    (total * (13 - month)) / 12
}
