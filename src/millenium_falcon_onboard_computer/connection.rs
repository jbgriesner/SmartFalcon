use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;
use serde_json::Result as jsonResult;

use rusqlite::{Connection,NO_PARAMS};
use rusqlite::Result as sqliteResult;

/// Just read and convert json files into jsonResult
pub fn read_data<P, D>(path: P) -> jsonResult<D>
where
    P: AsRef<Path>,
    for<'de> D: Deserialize<'de>
{
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let f = serde_json::from_reader(reader)?;
    Ok(f)
}

/// Store empire.json data
#[derive(Deserialize, Debug)]
pub struct EmpireData {
    pub countdown: u32,
    pub bounty_hunters: Vec<PlanetDanger>,
}

/// Store falcon.json data
#[derive(Deserialize, Debug)]
pub struct FalconData {
    pub autonomy: u32,
    pub departure: String,
    pub arrival: String,
    pub routes_db: String,
}

#[derive(Deserialize, Debug)]
pub struct PlanetDanger {
    pub planet: String,
    pub day: u32,
}

/// Get all edges between planets of the galaxy
pub fn get_routes() -> sqliteResult<Vec<(String,String,u32)>> {
    // We assume the sqlite database will always be in "data/universe.db" file, which "should have been" parametrized
    let conn = Connection::open("data/universe.db").unwrap();
    let mut stmt = conn.prepare("SELECT r.origin, r.destination, r.travel_time from routes r;").unwrap();
    let rows = stmt.query_map(NO_PARAMS, |row| Ok((row.get(0).unwrap(),row.get(1).unwrap(),row.get(2).unwrap()))).unwrap();

    let mut routes = Vec::new();
    for route_result in rows {
        routes.push(route_result.unwrap());
    }
    Ok(routes)
}
