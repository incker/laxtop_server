use chrono::NaiveDateTime;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;
use diesel::{Connection, ExpressionMethods, MysqlConnection, QueryDsl, QueryResult, RunQueryDsl};
use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::model::{InvoiceProduct, InvoiceStatus, RespInvoice, SpotSupplier};
use crate::schema::invoice;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewInvoice {
    #[serde(rename = "supplierId")]
    pub supplier_id: u32,
    pub products: InvoiceProduct,
}

impl NewInvoice {
    pub fn rocket_validate(
        &self,
        spot_id: u32,
        conn: &MysqlConnection,
    ) -> Result<(), Json<RespErrors>> {
        self.validate(spot_id, conn)
            .map_err(|error| Json(RespErrors::new_error(error)))
    }

    pub fn validate(&self, spot_id: u32, conn: &MysqlConnection) -> Result<(), (String, String)> {
        SpotSupplier::validate_ligament_exist(spot_id, self.supplier_id, conn)?;

        self.products
            .validate(self.supplier_id, conn)
            .map_err(|products_err| ("products".to_string(), products_err))
    }

    pub fn insert(self, user_id: u32, spot_id: u32, conn: &MysqlConnection) -> (u32, RespInvoice) {
        use crate::schema::invoice::dsl;
        let mut maybe_creation_id = NewInvoice::get_timestamp() as u32;

        // already validated
        let position_count = self.products.0.len() as u8;
        let mut invoice_id: u32 = 0;

        // we have some unique pairs for creation_id
        // that is why we try to insert incrementing creation_id until insert done
        loop {
            // transaction needed to take correct next invoice_id
            let res: QueryResult<usize> =
                conn.transaction::<usize, diesel::result::Error, _>(|| {
                    invoice_id = {
                        let last_invoice_id_res = invoice::table
                            .select(dsl::id)
                            .order(dsl::id.desc())
                            .first::<u32>(conn);

                        match last_invoice_id_res {
                            Ok(id) => id + 1,
                            Err(diesel::result::Error::NotFound) => 1,
                            Err(error) => panic!("{}", error),
                        }
                    };

                    let values = (
                        dsl::id.eq(invoice_id),
                        dsl::creation_id.eq(maybe_creation_id),
                        dsl::supplier_id.eq(self.supplier_id),
                        dsl::user_id.eq(user_id),
                        dsl::spot_id.eq(spot_id),
                        dsl::position_count.eq(position_count),
                        dsl::status.eq(InvoiceStatus::NotDelivered),
                        dsl::updated_at
                            .eq(NaiveDateTime::from_timestamp(maybe_creation_id as i64, 0)),
                    );

                    diesel::insert_into(invoice::table)
                        .values(&values)
                        .execute(conn)
                });

            match res {
                Ok(_) => break,
                Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                    maybe_creation_id += 1;
                }
                Err(error) => panic!("Some fail here: NewInvoice::insert, error: {:?} ", error),
            }
        }

        // insert products
        self.products
            .insert(invoice_id, conn)
            .unwrap_or_else(|_| panic!("error inserting products {:?}", &self.products));

        let NewInvoice {
            supplier_id,
            products,
        } = self;

        let resp_invoice = RespInvoice {
            creation_id: maybe_creation_id,
            spot_id,
            supplier_id,
            status: InvoiceStatus::Delivered,
            products,
        };

        (invoice_id, resp_invoice)
    }

    pub fn get_timestamp() -> u64 {
        use std::time::SystemTime;
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            _ => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}
