extern crate core;

use log::{debug, info};
use crate::lotto::Lotto;
use crate::console::LottoConsole;

pub mod console;
pub mod lotto;

fn main() {
    env_logger::init();
    info!("main called");
    println!("Hello Lotto Generator");

    let mut lotto_console = LottoConsole::new();
    lotto_console.start();

}
