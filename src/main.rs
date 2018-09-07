extern crate whatever;

use whatever::bot::get_bot_token;
use whatever::bot::AbsTBot;
use whatever::bot::TBot;

fn main() {
    let tbot = TBot::new(get_bot_token());
    tbot.run();
}
