#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod millenium_falcon_onboard_computer;
mod c_3po;

use std::io::prelude::*;
use std::collections::HashMap;
use crate::millenium_falcon_onboard_computer::{connection,galaxy};
use crate::c_3po::front;

fn main() {
    let falcon: connection::FalconData = connection::read_falcon_data("data/millenium-falcon.json").unwrap();
    println!("{:#?}", falcon);

    let routes = connection::get_routes().unwrap();

    let galaxy = galaxy::from_routes(routes, falcon.departure);
    println!("Routes in the galaxy looks like: {:?}", galaxy);

    //front::new().launch();
}
