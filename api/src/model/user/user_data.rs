use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::{InvoicePreview, Promo, SpotSupplierSequence, UserHistorySupplier, UserSpot};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    spots: Vec<SpotSupplierSequence>,
    suppliers: Vec<SupplierHeader>,
    #[serde(rename = "lastInvoicesPreview")]
    last_invoices_preview: InvoicePreview,
    #[serde(rename = "suppliersShift")]
    suppliers_shift: SuppliersShift,
    #[serde(rename = "promoIds")]
    promo_ids: Vec<u32>,
}

impl UserData {
    pub fn new(user_id: u32, conn: &MysqlConnection) -> Result<Self, (String, String)> {
        let (spots, supplier_ids, promo_ids) =
            UserData::get_user_spots_and_all_suppliers(user_id, conn);
        // select first spot as users main spot (any way no functionality to manage 2 spots for one user)
        let spot_id = if let Some(spot) = spots.first() {
            spot.id
        } else {
            return Err(("spotId".into(), "no spot for user".into()));
        };

        #[cfg(debug_assertions)]
        println!("{:?}", &spots);

        let (suppliers, suppliers_shift) = SupplierHeader::get_suppliers_info(&supplier_ids, conn);
        // invoices preview only for one spot
        let last_invoices_preview =
            InvoicePreview::get_last_invoice_preview_list(user_id, spot_id, conn, 5);
        Ok(UserData {
            spots,
            suppliers,
            suppliers_shift,
            last_invoices_preview,
            promo_ids,
        })
    }

    pub fn get_user_spots_and_all_suppliers(
        user_id: u32,
        conn: &MysqlConnection,
    ) -> (Vec<SpotSupplierSequence>, Vec<u32>, Vec<u32>) {
        let mut active_suppliers = Vec::new();

        let spot_ids = UserSpot::select_by_user(user_id, conn);
        let mut spots = Vec::with_capacity(spot_ids.len());

        for spot_id in spot_ids {
            let sequence = SpotSupplierSequence::get_sequence(user_id, spot_id, conn);
            for supplier_id in &sequence {
                active_suppliers.push(*supplier_id)
            }
            spots.push(SpotSupplierSequence {
                id: spot_id,
                sequence,
            })
        }

        active_suppliers.sort_unstable();
        active_suppliers.dedup();
        let promo_ids = Promo::select_suppliers_active_promo_ids(&active_suppliers, conn);

        let mut all_supplier_ids =
            UserHistorySupplier::get_user_history_supplier_ids(user_id, conn);

        for supplier_id in active_suppliers {
            all_supplier_ids.push(supplier_id);
        }

        all_supplier_ids.sort_unstable();
        all_supplier_ids.dedup();

        (spots, all_supplier_ids, promo_ids)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierHeader {
    id: u32,
    name: String,
}

impl SupplierHeader {
    pub fn get_suppliers_info(
        supplier_ids: &[u32],
        conn: &MysqlConnection,
    ) -> (Vec<SupplierHeader>, SuppliersShift) {
        use crate::schema::supplier::{self, dsl};

        let data: Vec<(u32, String, u8)> = supplier::table
            .select((dsl::id, dsl::name, dsl::shift))
            .filter(dsl::id.eq_any(supplier_ids))
            .load(conn)
            .expect("Some fail here: UserSpot::select_by_user :(");

        let mut suppliers: Vec<SupplierHeader> = Vec::with_capacity(supplier_ids.len());
        let mut shifts: Vec<(u32, u8)> = Vec::with_capacity(supplier_ids.len());

        for (id, name, shift) in data {
            suppliers.push(SupplierHeader { id, name });
            shifts.push((id, shift));
        }

        let suppliers_shift = SuppliersShift(shifts);
        (suppliers, suppliers_shift)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuppliersShift(pub Vec<(u32, u8)>);
