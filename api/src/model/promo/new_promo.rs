use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc};
use diesel::{
    sql_query, update, Connection, ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::model::Base64Image;
use crate::model::{
    ImageSizeValidation, Location, Promo, Session, SupplierStatus, ANNUAL_PROMO_AMOUNT,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPromo {
    #[serde(rename = "catId")]
    pub cat_id: u32,
    pub image: Base64Image,
}

impl NewPromo {
    pub fn save(&self, supplier_id: u32, conn: &MysqlConnection) -> Result<(), String> {
        let image_id = self
            .image
            .save_image(conn, ImageSizeValidation::Size((1080, 1920)))?;

        Promo::insert(supplier_id, self.cat_id, image_id, conn);

        Ok(())
    }
}
