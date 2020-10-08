fn some_telegram_test() {
    // tgbot::types::ParseMode
    let chat_id: i64 = 482231043;
    let api = get_telegram_api();
    send_msg(api, chat_id, "test: <code>test</code>").unwrap();
}
