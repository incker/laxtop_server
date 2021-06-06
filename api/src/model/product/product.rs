use std::collections::{HashMap, HashSet};

use diesel::{
    update, BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::{Supplier, Unit};
use crate::schema::product;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "product"]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub supplier_id: u32,
    pub key: String,
    pub unit: Unit,
    pub is_deleted: bool,
}

impl Product {
    pub fn validate_products_belong_to_supplier(
        supplier_id: u32,
        product_ids: &[u32],
        conn: &MysqlConnection,
    ) -> Result<(), String> {
        use crate::schema::product::dsl;

        let hash_set: HashSet<u32> = {
            let target = dsl::supplier_id
                .eq(supplier_id)
                .and(dsl::id.eq_any(product_ids));

            let data: Vec<u32> = product::table
                .select(dsl::id)
                .filter(target)
                .load::<u32>(conn)
                .expect("Some fail here: Product::validate_products_owner :(");

            data.into_iter().collect()
        };

        for product_id in product_ids {
            if !hash_set.contains(product_id) {
                return Err(format!(
                    "product_id: {} does not belong to supplier {}",
                    product_id, supplier_id
                ));
            }
        }

        Ok(())
    }

    pub fn get_product_names(
        product_ids: &[u32],
        supplier_id: u32,
        conn: &MysqlConnection,
    ) -> HashMap<u32, String> {
        use crate::schema::product::dsl;

        let target = dsl::id
            .eq_any(product_ids)
            .and(dsl::supplier_id.eq(supplier_id));

        let product_names: Vec<(u32, String)> = product::table
            .select((dsl::id, dsl::name))
            .filter(target)
            .load(conn)
            .expect("Some fail here: Product::get_products :(");

        let mut names_hash_map = HashMap::with_capacity(product_names.len());

        for (id, name) in product_names {
            names_hash_map.insert(id, name);
        }

        names_hash_map
    }

    ///
    pub fn replace_basic_products(supplier_id: u32, names: &[String], conn: &MysqlConnection) {
        use crate::schema::product::dsl;

        Product::add_basic_products(supplier_id, names, conn);

        let is_deleted = true; // WHERE NOT IN names
        let target = dsl::supplier_id
            .eq(supplier_id)
            .and(dsl::name.ne_all(names))
            .and(dsl::key.eq(""));

        update(product::table.filter(target))
            .set(dsl::is_deleted.eq(is_deleted))
            .execute(conn)
            .expect("Some fail here: Product::add_basic_products");

        Supplier::increment_supplier_shift(supplier_id, conn);
    }

    ///
    pub fn add_basic_products(supplier_id: u32, names: &[String], conn: &MysqlConnection) {
        use crate::schema::product::dsl;

        let is_deleted = false; // WHERE NOT IN names

        let target = dsl::supplier_id
            .eq(supplier_id)
            .and(dsl::name.eq_any(names))
            .and(dsl::key.eq(""));

        update(product::table.filter(target))
            .set(dsl::is_deleted.eq(is_deleted))
            .execute(conn)
            .expect("Some fail here: Product::add_basic_products");

        Supplier::increment_supplier_shift(supplier_id, conn);
    }

    ///
    pub fn count_basic_non_deleted(supplier_id: u32, conn: &MysqlConnection) -> i64 {
        use crate::schema::product::dsl;
        use diesel::dsl::count;

        let target = dsl::supplier_id
            .eq(supplier_id)
            .and(dsl::key.eq(""))
            .and(dsl::is_deleted.eq(false));

        product::table
            .select(count(dsl::supplier_id))
            .filter(target)
            .first(conn)
            .expect("Some fail here: Product::count_basic_non_deleted :(")
    }
}
