use crate::millenium_falcon_onboard_computer::{connection, falcon, galaxy, utils};
use rocket_contrib::json::Json;

/// Method called in static/main.js to compute the odds thanks to the back-end
#[post("/submit", format = "application/json", data = "<empire_json>")]
pub fn submit(empire_json: Json<connection::EmpireData>) -> String {
    let falcon: connection::FalconData =
        connection::read_data("data/millenium-falcon.json").unwrap();
    info!("{:#?}", falcon);

    let routes = connection::get_routes().unwrap();
    info!("{:?}", routes);

    let mut galaxy = galaxy::from_routes(routes);
    info!("Routes in the galaxy looks like: {:?}", galaxy);

    let paths = falcon::generate_paths(
        &galaxy,
        falcon.departure,
        falcon.arrival,
        empire_json.countdown,
        falcon.autonomy,
    );
    info!("ALL PATHS: {:?}", paths);

    if let Some(best_path) = utils::get_best_path(&mut galaxy, paths, &empire_json.bounty_hunters) {
        //format!("BEST PATH: {:?} with odds: {:?}%", best_path, (1.0_f64-best_path.odds)*100.0_f64)
        best_path
    } else {
        //format!("BEST PATH: {:?} with odds: {:?}%", String::from(" "), 0)
        format!("The odds are O%, it means that no path is possible in time.")
    }
}
