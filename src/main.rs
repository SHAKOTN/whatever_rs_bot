extern crate whatever;

use whatever::bot::AbsTBot;
use whatever::bot::TBot;
use whatever::bot::get_bot_token;

fn main() {
    let tbot = TBot::new(get_bot_token());
    tbot.run();
}