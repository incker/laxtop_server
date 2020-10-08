use std::ops::Deref;

use rocket_contrib::json::Json;

use crate::base::RespErrors;
use crate::guard::{DbConn, UserId};
use crate::model::{InvoicePreview, InvoiceStatus, NewInvoice, RespInvoice, Supplier};
use crate::telegram::telegram_send_invoice;

#[get("/get-last-invoice-preview-list/<spot_id>/<limit>")]
fn get_last_invoice_preview_list(
    user_id: UserId,
    spot_id: u32,
    limit: i64,
    db_conn: DbConn,
) -> Json<InvoicePreview> {
    // no need to validate route data
    let limit = if limit < 0 { 0 } else { limit };
    // validate?
    let invoice_preview_list = InvoicePreview::get_last_invoice_preview_list(
        user_id.into(),
        spot_id,
        db_conn.deref(),
        limit,
    );
    Json(invoice_preview_list)
}

#[get("/get-invoice/<spot_id>/<creation_id>")]
fn get_invoice(
    user_id: UserId,
    spot_id: u32,
    creation_id: u32,
    db_conn: DbConn,
) -> Result<Json<RespInvoice>, Json<RespErrors>> {
    // no need to validate route data
    RespInvoice::select_by_creation_id(user_id.0, spot_id, creation_id, db_conn.deref())
        .map(Json)
        .map_err(|_err| {
            let err_explain = format!(
                "creation_id: {} not exist for this user ({}) or spot ({})",
                creation_id, user_id.0, spot_id
            );
            Json(RespErrors {
                errors: vec![("creation_id".into(), err_explain)],
            })
        })
}

#[post(
    "/set-new-invoice/<spot_id>",
    format = "application/json",
    data = "<new_invoice>"
)]
fn set_new_invoice(
    user_id: UserId,
    spot_id: u32,
    new_invoice: Json<NewInvoice>,
    db_conn: DbConn,
) -> Result<Json<RespInvoice>, Json<RespErrors>> {
    user_id.rocket_validate_spot_access(spot_id, db_conn.deref())?;
    new_invoice.rocket_validate(spot_id, db_conn.deref())?;

    let (invoice_id, resp_invoice): (u32, RespInvoice) =
        new_invoice
            .into_inner()
            .insert(user_id.into(), spot_id, db_conn.deref());

    let sent =
        if let Some(chat_id) = Supplier::get_chat_id(resp_invoice.supplier_id, db_conn.deref()) {
            let invoice_human_readable = resp_invoice.human_readable(db_conn.deref());
            telegram_send_invoice(invoice_human_readable, chat_id)
                .map_err(|e| {
                    // todo: log somewhere
                    println!("{}", e);
                })
                .is_ok()
        } else {
            false
        };

    // decided to update status afterwards
    if sent {
        RespInvoice::update_status_by_id(invoice_id, InvoiceStatus::Delivered, db_conn.deref());
    }

    Ok(Json(resp_invoice))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_last_invoice_preview_list, get_invoice, set_new_invoice,]
}
