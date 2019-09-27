use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;
use serde_json::Result as jsonResult;

use rusqlite::{Connection,NO_PARAMS};
use rusqlite::Result as sqliteResult;

#[derive(Deserialize, Debug)]
pub struct FalconData {
    pub autonomy: u32,
    pub departure: String,
    pub arrival: String,
    pub routes_db: String,
}

fn from_universe_data() -> u32 {
    5
}

pub fn read_falcon_data<P: AsRef<Path>>(path: P) -> jsonResult<FalconData> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let f = serde_json::from_reader(reader)?;
    Ok(f)
}

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
