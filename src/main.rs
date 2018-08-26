extern crate whatever;

use std::{thread, time};

use whatever::{TBot, get_bot_token};


fn main() {
    let tbot = TBot::new(get_bot_token());
    let mut offset: i32 = 0;
    loop {
        let parsed_response = tbot.get_updates(&offset).unwrap();
        for result in parsed_response.result.unwrap() {
            // Telegram API requires to make getUpdates request with offset,
            // To drop old events
            offset = result.update_id + 1;
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}