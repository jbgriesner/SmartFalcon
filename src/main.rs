#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod c_3po;
mod millenium_falcon_onboard_computer;

use crate::c_3po::front;
use crate::millenium_falcon_onboard_computer::{connection,galaxy,falcon,utils};

fn main() {
    front::new().launch();




//   let falcon: connection::FalconData = connection::read_data("data/millenium-falcon.json").unwrap();
//   println!("{:#?}", falcon);
//
//   let routes = connection::get_routes().unwrap();
//   println!("{:?}", routes);
//
//   let mut galaxy = galaxy::from_routes(routes);
//   println!("Routes in the galaxy looks like: {:?}", galaxy);
//
//   let empire: connection::EmpireData = connection::read_data("data/empire.json").unwrap();
//   println!("{:#?}", empire);
//
//   let paths = falcon::generate_paths(&galaxy, falcon.departure, falcon.arrival, empire.countdown, falcon.autonomy);
//   println!("ALL PATHS: {:?}", paths);
//
//   if let Some(best_path) = utils::get_best_path(&mut galaxy, paths, &empire.bounty_hunters) {
//       println!("BEST PATH: {:?} with odds: {:?}%", best_path, (1.0_f64-best_path.odds)*100.0_f64);
//   } else {
//       println!("BEST PATH: {:?} with odds: {:?}%", String::from(" "), 0);
//   }
}
