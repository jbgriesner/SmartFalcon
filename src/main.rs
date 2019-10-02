#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod c_3po;
mod millenium_falcon_onboard_computer;

use crate::c_3po::front;
use crate::millenium_falcon_onboard_computer::{connection,galaxy,falcon,utils};

fn main() {
    front::new().launch();




}
