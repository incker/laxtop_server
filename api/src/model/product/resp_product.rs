use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::{Unit, UserHistoryProduct};
use crate::schema::product;

#[derive(Debug, Queryable, QueryableByName, PartialEq, Serialize, Deserialize)]
#[table_name = "product"]
pub struct RespProduct {
    pub id: u32,
    pub name: String,
    pub unit: Unit,
    #[serde(rename(serialize = "isDeleted"))]
    pub is_deleted: bool,
}

impl RespProduct {
    pub fn get_products(user_id: u32, supplier_id: u32, conn: &MysqlConnection) -> Vec<Self> {
        use crate::schema::product::dsl;

        let product_ids = UserHistoryProduct::get_user_history_product_ids(user_id, conn);

        /*
           SELECT product.id, product.supplier_id, product.name
           FROM product
           WHERE product.supplier_id IN (1, 2, 3)
             AND (product.id IN (1, 2, 3) OR product.is_deleted = false)
        */

        let target = dsl::supplier_id
            .eq(supplier_id)
            .and(dsl::id.eq_any(&product_ids).or(dsl::is_deleted.eq(false)));

        let products: Vec<RespProduct> = product::table
            .filter(target)
            .select((dsl::id, dsl::name, dsl::unit, dsl::is_deleted))
            .load(conn)
            .expect("Some fail here: SpotSupplierSequence::select_supplier_ids_by_user_id :(");

        products
    }

    pub fn get_supplier_products(supplier_id: u32, conn: &MysqlConnection) -> Vec<Self> {
        use crate::schema::product::dsl;

        let target = dsl::supplier_id
            .eq(supplier_id)
            .and(dsl::is_deleted.eq(false));

        product::table
            .select((dsl::id, dsl::name, dsl::unit, dsl::is_deleted))
            .filter(target)
            .load::<RespProduct>(conn)
            .expect("Error loading products")
    }

    pub fn get_products_by_ids(ids: &[u32], conn: &MysqlConnection) -> Vec<Self> {
        use crate::schema::product::dsl;

        let target = dsl::id.eq_any(ids);

        let query = product::table
            .select((dsl::id, dsl::name, dsl::unit, dsl::is_deleted))
            .filter(target);

        //println!("{:?}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query));
        query
            .load::<RespProduct>(conn)
            .expect("Error loading products")
    }
}
