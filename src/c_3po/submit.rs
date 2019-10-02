use rocket::Data;
use rocket::http::ContentType;
use crate::millenium_falcon_onboard_computer::{connection,galaxy,falcon,utils};
use rocket_contrib::json::{Json, JsonValue};

#[post("/submit", format = "application/json", data = "<empire_json>")]
pub fn submit(empire_json: Json<connection::EmpireData>) -> String {
    let falcon: connection::FalconData = connection::read_data("data/millenium-falcon.json").unwrap();
    println!("{:#?}", falcon);

    let routes = connection::get_routes().unwrap();
    println!("{:?}", routes);

    let mut galaxy = galaxy::from_routes(routes);
    println!("Routes in the galaxy looks like: {:?}", galaxy);

    //let empire: connection::EmpireData = connection::read_data("data/empire.json").unwrap();
    //println!("{:#?}", empire);

    let paths = falcon::generate_paths(&galaxy, falcon.departure, falcon.arrival, empire_json.countdown, falcon.autonomy);
    println!("ALL PATHS: {:?}", paths);

    if let Some(best_path) = utils::get_best_path(&mut galaxy, paths, &empire_json.bounty_hunters) {
        format!("BEST PATH: {:?} with odds: {:?}%", best_path, (1.0_f64-best_path.odds)*100.0_f64)
    } else {
        format!("BEST PATH: {:?} with odds: {:?}%", String::from(" "), 0)
    }
}
