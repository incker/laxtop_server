use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::{RespErrors, ValidateFormatter};
use crate::guard::{DbConn, SupplierId};
use crate::model::SupplierInfo;

#[get("/get-supplier-info")]
fn get_supplier_info(supplier_id: SupplierId, db_conn: DbConn) -> Json<SupplierInfo> {
    let supplier_info = SupplierInfo::select_by_id(supplier_id.into(), db_conn.deref());
    Json(supplier_info)
}

#[post(
    "/set-supplier-info",
    format = "application/json",
    data = "<supplier_info>"
)]
fn set_supplier_info(
    supplier_id: SupplierId,
    supplier_info: Json<SupplierInfo>,
    db_conn: DbConn,
) -> Result<Json<SupplierInfo>, Json<RespErrors>> {
    let mut info = supplier_info.into_inner();
    info.clean();
    info.run_validator().map_err(Json)?;
    info.update(supplier_id.0, db_conn.deref());
    Ok(get_supplier_info(supplier_id, db_conn))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_supplier_info, set_supplier_info,]
}
