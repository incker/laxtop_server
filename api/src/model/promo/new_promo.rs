use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};
use diesel::{Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl, sql_query, update};

use crate::model::{ANNUAL_PROMO_AMOUNT, ImageSizeValidation, Location, Promo, Session, SupplierStatus};
use crate::model::Base64Image;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPromo {
    #[serde(rename = "catId")]
    pub cat_id: u32,
    pub image: Base64Image,
}

impl NewPromo {
    pub fn save(&self, supplier_id: u32, conn: &MysqlConnection) -> Result<(), String> {
        let image_id = self.image
            .save_image(conn, ImageSizeValidation::Size((1080, 1920)))?;

        Promo::insert(supplier_id, self.cat_id, image_id, conn);

        Ok(())
    }
}
