use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, insert_or_ignore_into, JoinOnDsl, MysqlConnection,
    QueryDsl, RunQueryDsl, update,
};

use crate::model::{Image, Location, Session, SupplierStatus};
use crate::schema::supplier;

const PROMO_LIFETIME_DAYS_AMOUNT: i64 = 14;

#[derive(Debug, Serialize, Deserialize)]
pub struct Promo {
    pub id: u32,
    #[serde(rename = "supplierId")]
    pub supplier_id: u32,
    #[serde(rename = "catId")]
    pub cat_id: u32,
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}


impl Promo {
    pub fn select_suppliers_active_promo_ids(supplier_ids: &[u32], conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::{promo, image};
        promo::table
            .filter(promo::dsl::supplier_id.eq_any(supplier_ids)
                .and(promo::dsl::created_at.ge(Promo::active_promos_creation_date()))
                .and(promo::dsl::image_id.ne(0))
                .and(promo::dsl::cat_id.ne(0))
            )
            .select(promo::dsl::id)
            .load::<u32>(conn)
            .unwrap()
    }


    pub fn select_day_old_promo_id(supplier_id: u32, conn: &MysqlConnection) -> Option<u32> {
        use crate::schema::promo;
        let one_day_ago = Utc::now().naive_utc().checked_sub_signed(Duration::days(1)).unwrap();
        promo::table
            .filter(promo::dsl::supplier_id.eq(supplier_id)
                .and(promo::dsl::created_at.ge(one_day_ago))
                .and(promo::dsl::image_id.ne(0))
            )
            .select(promo::dsl::id)
            .first::<u32>(conn)
            .ok()
    }


    pub fn select_suppliers_active_promos(supplier_ids: &[u32], conn: &MysqlConnection) -> Vec<Self> {
        use crate::schema::{promo, image};

        promo::table
            .inner_join(image::table.on(promo::dsl::image_id.eq(image::dsl::id)))
            .filter(promo::dsl::supplier_id.eq_any(supplier_ids)
                .and(promo::dsl::created_at.ge(Promo::active_promos_creation_date()))
            )
            .select((promo::dsl::id, promo::dsl::supplier_id, promo::dsl::cat_id, promo::dsl::created_at, image::dsl::dir, image::dsl::dir2, image::dsl::hash))
            .load::<(u32, u32, u32, NaiveDateTime, String, String, String)>(conn)
            .unwrap()
            .into_iter()
            .map(Promo::row_into_promo)
            .collect::<Vec<_>>()
    }

    pub fn row_into_promo((id, supplier_id, cat_id, created_at, dir, dir2, hash): (u32, u32, u32, NaiveDateTime, String, String, String)) -> Promo {
        Promo {
            id,
            supplier_id,
            cat_id,
            url: Image::build_url(&dir, &dir2, &hash),
            created_at,
        }
    }

    pub fn active_promos_creation_date() -> NaiveDateTime {
        Utc::now().naive_utc().checked_sub_signed(Duration::days(PROMO_LIFETIME_DAYS_AMOUNT)).unwrap()
    }

    pub fn count_annual_supplier_promos(supplier_id: u32, conn: &MysqlConnection) -> i32 {
        use crate::schema::promo::{self, dsl};
        let year = Promo::current_year();
        // month is 2 cause no need to shrink promo amount for suppliers registered in january
        let date = NaiveDate::from_ymd(year, 1, 1).and_hms(0, 0, 0);

        let count = promo::table
            .select(diesel::dsl::count(dsl::id))
            .filter(dsl::supplier_id.eq(supplier_id).and(dsl::created_at.ge(date)))
            .first::<i64>(conn)
            .unwrap();

        count as i32
    }

    pub fn insert(supplier_id: u32, cat_id: u32, image_id: u32, conn: &MysqlConnection) {
        use crate::schema::promo::{self, dsl};
        diesel::insert_into(promo::table)
            .values((
                dsl::supplier_id.eq(supplier_id),
                dsl::cat_id.eq(cat_id),
                dsl::image_id.eq(image_id),
            ))
            .execute(conn)
            .unwrap();
    }

    pub fn insert_dummy_promo(supplier_id: u32, conn: &MysqlConnection) {
        use crate::schema::promo::{self, dsl};
        let year = Promo::current_year();
        // month is 2 cause no need to shrink promo amount for suppliers registered in january
        let date = NaiveDate::from_ymd(year, 1, 1).and_hms(0, 0, 0);

        diesel::insert_into(promo::table)
            .values((
                dsl::supplier_id.eq(supplier_id),
                dsl::cat_id.eq(0),
                dsl::image_id.eq(0),
                dsl::created_at.eq(date),
            ))
            .execute(conn)
            .unwrap();
    }

    fn current_year() -> i32 {
        let (_is_common_era, year) = Utc::now().year_ce();
        year as i32
    }
}