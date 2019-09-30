
use crate::millenium_falcon_onboard_computer::{connection,galaxy::Galaxy,falcon::Path};

pub fn odds_from_k(count: u32) -> f64 {
    let mut odds = 0.1_f64;
    for k in 2..count+1 {
        odds += 9_u32.pow(k-1) as f64 / 10_u32.pow(k) as f64;
    }
    odds
}

pub fn get_best_path(galaxy: &mut Galaxy, mut paths: Vec<Path>, bounty_hunters: &Vec<connection::PlanetDanger>) -> Option<Path> {

    for path in &mut paths {
        path.compute_odds(bounty_hunters, galaxy);
    }

    paths.sort_by(|p1, p2| p1.odds.partial_cmp(&p2.odds).unwrap());

    if paths.len() > 0 {
        Some(paths[0].clone())
    } else {
        None
    }
}
