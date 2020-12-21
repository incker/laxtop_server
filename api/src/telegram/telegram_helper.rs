use crate::model::InvoiceHumanReadable;
use crate::route::bot::telegram::{get_telegram_api, send_msg};
use crate::telegram::TextMonoRectangle;

pub fn telegram_send_invoice(
    invoice_human_readable: InvoiceHumanReadable,
    chat_id: i64,
) -> Result<(), String> {
    let api = get_telegram_api();
    let formatted_message = format_invoice(invoice_human_readable);
    send_msg(api, chat_id, &formatted_message)
}

fn format_invoice(invoice_human_readable: InvoiceHumanReadable) -> String {
    let InvoiceHumanReadable { spot_info, data } = invoice_human_readable;
    let mut message = spot_info;
    message.push_str("\n\n");
    format_telegram_invoice(data, &mut message);
    message
}

fn format_telegram_invoice(data: Vec<(String, u32)>, message: &mut String) {
    let text_mono_rectangle = TextMonoRectangle::new(33);

    message.push_str("<code>");

    for (name, amount) in data {
        message.push_str("__________________________________|____");
        message.push('\n');
        let mut amount_printed = false;

        for lines in text_mono_rectangle.chunk_text(&name) {
            message.push_str(lines.0);
            message.push_str(lines.1);
            message.push('|');
            if !amount_printed {
                amount_printed = true;
                message.push(' ');
                message.push_str(&amount.to_string());
            }

            message.push('\n');
        }

        // that case when name is empty
        if !amount_printed {
            message.push_str("UNKNOWN");
            let spaces = &text_mono_rectangle
                .spaces_amount(text_mono_rectangle.line_char_limit - "UNKNOWN".len());
            message.push_str(spaces);
            message.push_str(" | ");
            message.push_str(&amount.to_string());
        }
    }

    message.push_str("</code>");
}
