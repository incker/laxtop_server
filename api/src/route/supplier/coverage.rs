use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{DbConn, SupplierId};
use crate::model::SupplierCoverage;

#[post("/set-coverage", format = "application/json", data = "<coverage>")]
fn set_coverage(
    supplier_id: SupplierId,
    coverage: Json<Vec<(f32, f32)>>,
    db_conn: DbConn,
) -> Result<Json<Vec<(f32, f32)>>, Json<RespErrors>> {
    let supplier_coverage = SupplierCoverage {
        supplier_id: supplier_id.0,
        coverage: coverage.into_inner(),
    };

    supplier_coverage.rocket_validate()?;
    supplier_coverage.insert(db_conn.deref());
    Ok(get_coverage(supplier_id, db_conn))
}

#[get("/get-coverage")]
fn get_coverage(supplier_id: SupplierId, db_conn: DbConn) -> Json<Vec<(f32, f32)>> {
    Json(SupplierCoverage::select_coverage(
        supplier_id.into(),
        db_conn.deref(),
    ))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![set_coverage, get_coverage]
}
