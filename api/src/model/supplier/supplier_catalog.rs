use diesel::MysqlConnection;

use crate::model::RespProduct;

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierCatalog {
    #[serde(rename(serialize = "supplierId"))]
    supplier_id: u32,
    products: Vec<RespProduct>,
    update: u32,
}

impl SupplierCatalog {
    pub fn new(user_id: u32, supplier_id: u32, conn: &MysqlConnection) -> Self {
        let products = RespProduct::get_products(user_id, supplier_id, conn);

        SupplierCatalog {
            supplier_id,
            products,
            update: 0,
        }
    }
}
