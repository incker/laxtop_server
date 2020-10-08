use diesel::{
    update, BoolExpressionMethods, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::{InvoiceHumanReadable, InvoiceProduct, InvoiceStatus, Product};
use crate::schema::invoice;

#[derive(Debug, Serialize, Deserialize)]
pub struct RespInvoice {
    #[serde(rename = "creationId")]
    pub creation_id: u32,
    #[serde(rename = "spotId")]
    pub spot_id: u32,
    #[serde(rename = "supplierId")]
    pub supplier_id: u32,
    pub status: InvoiceStatus,
    pub products: InvoiceProduct,
}

impl RespInvoice {
    pub fn select_by_creation_id(
        user_id: u32,
        spot_id: u32,
        creation_id: u32,
        conn: &MysqlConnection,
    ) -> Result<Self, diesel::result::Error> {
        use crate::schema::invoice::dsl;

        let target = dsl::user_id.eq(user_id).and(
            dsl::spot_id
                .eq(spot_id)
                .and(dsl::creation_id.eq(creation_id)),
        );

        let (invoice_id, creation_id, supplier_id, status): (u32, u32, u32, InvoiceStatus) =
            invoice::table
                .select((dsl::id, dsl::creation_id, dsl::supplier_id, dsl::status))
                .filter(target)
                .order(dsl::creation_id.desc())
                .first(conn)?;

        let products = InvoiceProduct::select_by_invoice_id(invoice_id, conn);

        Ok(RespInvoice {
            creation_id,
            spot_id,
            supplier_id,
            status,
            products,
        })
    }

    pub fn update_status(&self, status: InvoiceStatus, conn: &MysqlConnection) {
        use crate::schema::invoice::dsl;

        let target = invoice::table.filter(
            dsl::creation_id
                .eq(self.creation_id)
                .and(dsl::supplier_id.eq(self.supplier_id))
                .and(dsl::spot_id.eq(self.spot_id)),
        );
        let _res = update(target).set(dsl::status.eq(status)).execute(conn);
    }

    pub fn update_status_by_id(invoice_id: u32, status: InvoiceStatus, conn: &MysqlConnection) {
        use crate::schema::invoice::dsl;

        update(invoice::table.filter(dsl::id.eq(invoice_id)))
            .set(dsl::status.eq(status))
            .execute(conn)
            .unwrap();
    }

    pub fn get_product_ids(&self) -> Vec<u32> {
        let mut ids = Vec::with_capacity(self.products.0.len());
        for (id, _) in &self.products.0 {
            ids.push(*id);
        }
        ids
    }

    pub fn human_readable(&self, conn: &MysqlConnection) -> InvoiceHumanReadable {
        let product_ids = self.get_product_ids();
        let mut product_names = Product::get_product_names(&product_ids, self.supplier_id, conn);

        let spot_info = "Васина точка на усратой".to_string();

        let mut data: Vec<(String, u32)> = Vec::with_capacity(product_ids.len());

        for (id, amount) in &self.products.0 {
            let name = product_names
                .remove(id)
                .unwrap_or_else(|| "NOT_FOUND".to_string());
            data.push((name, *amount));
        }

        InvoiceHumanReadable { spot_info, data }
    }

    pub fn select_by_creation_ids(
        user_id: u32,
        spot_id: u32,
        creation_ids: &[u32],
        conn: &MysqlConnection,
    ) -> Vec<Self> {
        use crate::schema::invoice::dsl;

        if creation_ids.is_empty() {
            return vec![];
        }

        let target = dsl::user_id.eq(user_id).and(
            dsl::spot_id
                .eq(spot_id)
                .and(dsl::creation_id.eq_any(creation_ids)),
        );

        let rows: Vec<(u32, u32, u32, InvoiceStatus)> = invoice::table
            .select((dsl::id, dsl::creation_id, dsl::supplier_id, dsl::status))
            .filter(target)
            .order(dsl::creation_id.desc())
            .load(conn)
            .unwrap();

        let mut invoice_data = {
            let mut invoice_ids = Vec::with_capacity(rows.len());
            for (id, _creation_id, _supplier_id, _status) in &rows {
                invoice_ids.push(*id);
            }
            InvoiceProduct::select_by_invoice_ids(&invoice_ids, conn)
        };

        let mut resp_invoices = Vec::with_capacity(rows.len());

        for (id, creation_id, supplier_id, status) in rows {
            let resp_invoice = RespInvoice {
                creation_id,
                spot_id,
                supplier_id,
                status,
                products: invoice_data.remove(&id).unwrap_or_default(),
            };
            resp_invoices.push(resp_invoice);
        }

        resp_invoices
    }
}
