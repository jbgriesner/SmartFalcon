#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
extern crate simple_logger;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use log::Level;

mod c_3po;
mod millenium_falcon_onboard_computer;

use crate::c_3po::front;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    front::new().launch();
}
