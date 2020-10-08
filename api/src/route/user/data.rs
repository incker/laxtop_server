use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{DbConn, UserId};
use crate::model::{SpotSupplierSequence, SupplierCatalog, UserData, UserSpot};

#[get("/get-user-data")]
fn get_user_data(user_id: UserId, db_conn: DbConn) -> Result<Json<UserData>, Json<RespErrors>> {
    UserData::new(user_id.into(), db_conn.deref())
        .map(Json)
        .map_err(|error| Json(RespErrors::new_error(error)))
}

#[get("/get-supplier-catalog/<supplier_id>")]
fn get_supplier_catalog(
    user_id: UserId,
    supplier_id: u32,
    db_conn: DbConn,
) -> Json<SupplierCatalog> {
    Json(SupplierCatalog::new(
        user_id.into(),
        supplier_id,
        db_conn.deref(),
    ))
}

#[post(
    "/set-supplier-sequence",
    format = "application/json",
    data = "<spot_supplier_sequence>"
)]
fn set_supplier_sequence(
    user_id: UserId,
    spot_supplier_sequence: Json<SpotSupplierSequence>,
    db_conn: DbConn,
) -> Result<Json<SpotSupplierSequence>, Json<RespErrors>> {
    let SpotSupplierSequence {
        id: spot_id,
        sequence,
    } = spot_supplier_sequence.into_inner();
    UserSpot::validate_ligament(user_id.0, spot_id, db_conn.deref())
        .map_err(|err| Json(RespErrors::new(vec![err])))?;
    SpotSupplierSequence::set_sequence(user_id.0, spot_id, &sequence, db_conn.deref());
    Ok(Json(SpotSupplierSequence::select(
        user_id.into(),
        spot_id,
        db_conn.deref(),
    )))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_user_data, get_supplier_catalog, set_supplier_sequence,]
}
