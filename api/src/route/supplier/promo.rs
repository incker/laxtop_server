use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{DbConn, SupplierId};
use crate::model::{
    Base64Image, DataWrapper, ImageSizeValidation, NewPromo, Promo, PromoGroup, SupplierPromoData,
};

#[post("/set-promo", format = "application/json", data = "<new_promo>")]
fn set_promo(
    supplier_id: SupplierId,
    new_promo: Json<NewPromo>,
    db_conn: DbConn,
) -> Result<Json<DataWrapper<bool>>, Json<RespErrors>> {
    if let Some(_id) = Promo::select_day_old_promo_id(supplier_id.0, db_conn.deref()) {
        return Err(Json(RespErrors::new_error((
            "promo".into(),
            "Add 2 promos in 24 hours is not allowed".into(),
        ))));
    }
    new_promo
        .save(supplier_id.0, db_conn.deref())
        .map_err(|err| Json(RespErrors::new_error(("image".into(), err))))?;

    Ok(Json(DataWrapper::new(true)))
}

#[get("/get-promo-data")]
fn get_promo_data(supplier_id: SupplierId, db_conn: DbConn) -> Json<SupplierPromoData> {
    Json(SupplierPromoData::new(supplier_id.0, db_conn.deref()))
}

#[get("/get-promo-cats")]
fn get_promo_cats(_supplier_id: SupplierId, db_conn: DbConn) -> Json<DataWrapper<Vec<PromoGroup>>> {
    let res = PromoGroup::select_all(db_conn.deref());
    Json(DataWrapper::new(res))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![set_promo, get_promo_data, get_promo_cats]
}
