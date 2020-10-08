use diesel::{AsExpression, BoolExpressionMethods, delete, Expression, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::promo::cats::Cat;
use crate::schema::user_promo_cat;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "user_promo_cat"]
pub struct UserPromoCat {
    pub user_id: u32,
    pub promo_cat_id: u32,
}

impl UserPromoCat {
    pub fn select_by_user(user_id: u32, conn: &MysqlConnection) -> Vec<u32> {
        use crate::schema::user_promo_cat::dsl;

        user_promo_cat::table
            .select(dsl::promo_cat_id)
            .filter(dsl::user_id.eq(user_id))
            .load(conn)
            .expect("Some fail here: UserPromoCat::select_by_user :(")
    }

    pub fn insert_values(user_id: u32, promo_cat_id: u32, conn: &MysqlConnection) {
        use crate::schema::user_promo_cat::dsl;
        let _res = diesel::insert_into(user_promo_cat::table)
            .values((dsl::user_id.eq(user_id), dsl::promo_cat_id.eq(promo_cat_id)))
            .execute(conn);
    }


    pub fn insert_promo_cat_ids(user_id: u32, promo_cat_ids: &[u32], conn: &MysqlConnection) {
        use crate::schema::user_promo_cat::dsl;

        let promo_cat_ids = Cat::check_ids_existence(promo_cat_ids, conn);

        let target = dsl::user_id.eq(user_id).and(dsl::promo_cat_id.ne_all(&promo_cat_ids));
        let _res = delete(user_promo_cat::table.filter(target)).execute(conn);

        let values: Vec<UserPromoCat> = promo_cat_ids.into_iter().map(|promo_cat_id| {
            UserPromoCat { user_id, promo_cat_id }
        }).collect();

        let _res = diesel::insert_or_ignore_into(user_promo_cat::table)
            .values(&values)
            .execute(conn);
    }


    pub fn validate_ligament(
        user_id: u32,
        promo_cat_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), (String, String)> {
        use crate::schema::user_promo_cat::dsl;
        user_promo_cat::table
            .select(dsl::promo_cat_id)
            .filter(dsl::user_id.eq(user_id))
            .first::<u32>(conn)
            .map(|_| ())
            .map_err(|_| {
                (
                    "user_promo_cat".to_string(),
                    format!("user_id {} does not own promo_cat_id {}", user_id, promo_cat_id),
                )
            })
    }
}
