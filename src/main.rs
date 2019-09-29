#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod millenium_falcon_onboard_computer;
mod c_3po;

use crate::millenium_falcon_onboard_computer::{connection,galaxy,falcon};
use crate::c_3po::front;

fn main() {
    let falcon: connection::FalconData = connection::read_data("data/millenium-falcon.json").unwrap();
    println!("{:#?}", falcon);

    let empire: connection::EmpireData = connection::read_data("data/empire.json").unwrap();
    println!("{:#?}", empire);

    let routes = connection::get_routes().unwrap();
    println!("{:?}", routes);

    let galaxy = galaxy::from_routes(routes);
    println!("Routes in the galaxy looks like: {:?}", galaxy);

    let paths = falcon::generate_paths(&galaxy, falcon.departure, falcon.arrival, empire.countdown, falcon.autonomy);
    println!("ALL PATHS: {:?}", paths);
    //front::new().launch();
}
