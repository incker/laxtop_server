use diesel::MysqlConnection;

use crate::model::Promo;

pub const ANNUAL_PROMO_AMOUNT: i32 = 20;

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierPromoData {
    total: i32,
    wasted: i32,
    #[serde(rename = "activePromos")]
    active_promos: Vec<Promo>,
}

impl SupplierPromoData {
    pub fn new(supplier_id: u32, conn: &MysqlConnection) -> Self {
        SupplierPromoData {
            total: ANNUAL_PROMO_AMOUNT,
            wasted: Promo::count_annual_supplier_promos(supplier_id, conn),
            active_promos: Promo::select_suppliers_active_promos(&[supplier_id], conn),
        }
    }
}
