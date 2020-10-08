use std::collections::{HashMap, HashSet};

use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::Product;
use crate::schema::invoice_product;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceProduct(pub Vec<(u32, u32)>);

impl InvoiceProduct {
    /// validate duplicates
    /// validate each amount != 0
    /// validate product positions count > 0 and <= 255
    /// validate products belonging to supplier
    pub fn validate(&self, supplier_id: u32, conn: &MysqlConnection) -> Result<(), String> {
        let product_ids: Vec<u32> = {
            let rows_len = self.0.len();
            if rows_len == 0 {
                return Err("There was no products provided".to_string());
            }
            if rows_len > 255 {
                return Err(
                    "Sorry, you can not submit more than 255 positions in one invoice".to_string(),
                );
            }

            let mut product_ids = Vec::with_capacity(rows_len);
            let mut hash_set: HashSet<u32> = HashSet::with_capacity(rows_len);

            for (product_id, amount) in &self.0 {
                if *amount == 0 {
                    return Err(format!(
                        "Products can not be zero amount (product_id: {})",
                        product_id
                    ));
                }

                if hash_set.contains(product_id) {
                    return Err(format!(
                        "Product can not be set twice (product_id: {})",
                        product_id
                    ));
                } else {
                    hash_set.insert(*product_id);
                    product_ids.push(*product_id);
                }
            }
            product_ids
        };

        Product::validate_products_belong_to_supplier(supplier_id, &product_ids, conn)
    }

    pub fn select_by_invoice_ids(
        invoice_ids: &[u32],
        conn: &MysqlConnection,
    ) -> HashMap<u32, Self> {
        use crate::schema::invoice_product::dsl;

        let mut invoice_data: HashMap<u32, InvoiceProduct> =
            HashMap::with_capacity(invoice_ids.len());

        let rows: Vec<(u32, u32, u32)> = invoice_product::table
            .select((dsl::invoice_id, dsl::product_id, dsl::amount))
            .filter(dsl::invoice_id.eq_any(invoice_ids))
            .load(conn)
            .unwrap();

        for (invoice_id, product_id, amount) in rows {
            if let Some(invoice_product) = invoice_data.get_mut(&invoice_id) {
                invoice_product.0.push((product_id, amount));
            } else {
                invoice_data.insert(invoice_id, InvoiceProduct(vec![(product_id, amount)]));
            };
        }

        invoice_data
    }

    pub fn select_by_invoice_id(invoice_id: u32, conn: &MysqlConnection) -> Self {
        use crate::schema::invoice_product::dsl;

        let rows: Vec<(u32, u32)> = invoice_product::table
            .select((dsl::product_id, dsl::amount))
            .filter(dsl::invoice_id.eq(invoice_id))
            .load(conn)
            .unwrap();

        InvoiceProduct(rows)
    }

    pub fn insert(
        &self,
        invoice_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::invoice_product::dsl;

        let mut values_batch = Vec::with_capacity(self.0.len());

        for (product_id, amount) in &self.0 {
            let values = (
                dsl::invoice_id.eq(invoice_id),
                dsl::product_id.eq(product_id),
                dsl::amount.eq(amount),
            );
            values_batch.push(values);
        }

        diesel::insert_into(invoice_product::table)
            .values(&values_batch)
            .execute(conn)
            .map(|_| ())
    }
}
