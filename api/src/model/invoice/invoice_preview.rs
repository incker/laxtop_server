use diesel::{BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

use crate::model::NewInvoice;
use crate::schema::invoice;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePreview {
    pub data: Vec<(u32, u32, u8)>,
}

impl InvoicePreview {
    pub fn get_last_invoice_preview_list(
        user_id: u32,
        spot_it: u32,
        conn: &MysqlConnection,
        limit: i64,
    ) -> InvoicePreview {
        use crate::schema::invoice::dsl;
        let expired_date_time: u32 = {
            let tree_years: u32 = 365 * 24 * 60 * 60;
            NewInvoice::get_timestamp() as u32 - tree_years
        };

        let data: Vec<(u32, u32, u8)> = invoice::table
            .select((dsl::creation_id, dsl::supplier_id, dsl::position_count))
            .filter(
                dsl::user_id
                    .eq(user_id)
                    .and(dsl::spot_id.eq(spot_it))
                    .and(dsl::creation_id.gt(expired_date_time)),
            )
            .order(dsl::creation_id.desc())
            .limit(if limit == 0 { 100_000 } else { limit })
            .load(conn)
            .unwrap();

        InvoicePreview { data }
    }
}
