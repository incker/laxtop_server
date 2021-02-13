use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{DbConn, UserId};
use crate::model::{DataWrapper, Promo, SpotSupplierSequence, SupplierInfo, UserSpot};

#[get("/get-supplier-info/<supplier_id>")]
fn get_supplier_info(
    user_id: UserId,
    supplier_id: u32,
    db_conn: DbConn,
) -> Result<Json<SupplierInfo>, Json<RespErrors>> {
    user_id.rocket_validate_supplier_access(supplier_id, db_conn.deref())?;

    let supplier_info = SupplierInfo::select_by_id(supplier_id, db_conn.deref());
    Ok(Json(supplier_info))
}

#[get("/get-promos")]
fn get_promos(user_id: UserId, db_conn: DbConn) -> Json<DataWrapper<Vec<Promo>>> {
    let user_id = user_id.0;

    let supplier_ids = {
        let mut supplier_ids = Vec::new();

        for spot_id in UserSpot::select_by_user(user_id, db_conn.deref()) {
            let sequence = SpotSupplierSequence::get_sequence(user_id, spot_id, db_conn.deref());
            for supplier_id in &sequence {
                supplier_ids.push(*supplier_id)
            }
        }
        supplier_ids.sort_unstable();
        supplier_ids.dedup();
        supplier_ids
    };

    let promos = Promo::select_suppliers_active_promos(&supplier_ids, db_conn.deref());

    Json(DataWrapper::new(promos))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_supplier_info, get_promos]
}
