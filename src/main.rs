extern crate whatever;

use whatever::{TBot, get_bot_token};


fn main() {
    let tbot = TBot::new(get_bot_token());
    tbot.run();
}