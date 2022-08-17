use log::{debug, info};
//use crate::lotto

pub mod simple_console;
pub mod lotto;

fn main() {
    env_logger::init();
    info!("main called");
    println!("Hello Lotto Generator");

    let mut lotto1 = lotto::Lotto::new();
    lotto1.fill_rand_nums();
    debug!("{lotto1:?}");

    let mut lotto2 = lotto::Lotto::new();
    lotto2.fill_manual((1,2,3,4,5,6));
    debug!("{lotto2:?}");

    let result = lotto1.diff(&lotto2);
    debug!("{result}");

}
