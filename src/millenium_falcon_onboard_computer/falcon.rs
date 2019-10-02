use crate::millenium_falcon_onboard_computer::galaxy::Galaxy;
use crate::millenium_falcon_onboard_computer::{connection, utils};
use std::collections::HashMap;
use std::fmt;

/// Store odds, planets and time corresponding to path explored in the galaxy
/// 'planets' Vec items are pairs of planet id and visit time
#[derive(Default, Clone)]
pub struct Path {
    pub planets: Vec<(u32, u32)>,
    pub odds: f64,
}

/// Just implement Debug for dev purpose
impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pl, time) in self.planets.iter() {
            write!(f, "{:?}[{:?}] -> ", pl, time)?;
        }
        Ok(())
    }
}

impl Path {
    fn new() -> Path {
        Path {
            planets: Vec::new(),
            ..Default::default()
        }
    }

    pub fn compute_odds(
        &mut self,
        bounty_hunters: &Vec<connection::PlanetDanger>,
        galaxy: &mut Galaxy,
    ) {
        let mut count = 0;
        for planet_danger in bounty_hunters {
            if self.planets.contains(&(
                galaxy.get_planet_id(planet_danger.planet.clone()),
                planet_danger.day,
            )) {
                count += 1;
            }
        }
        self.odds = utils::odds_from_k(count)
    }

    fn add_planet_time(&mut self, planet_id: u32, time: u32) {
        self.planets.push((planet_id, time));
    }

    pub fn display(&self, galaxy: &Galaxy) -> String {
        let mut result = format!(
            "The odds are {:?}%. The path corresponding is: ",
            (1.0_f64 - self.odds) * 100.0_f64
        );
        for (pl, _) in self.planets.iter() {
            result.push_str(&format!(
                "{:?} -> ",
                galaxy.planets.get(&pl).unwrap().name.clone()
            ));
        }
        result.truncate(result.len() - 4);
        result
    }
}

/// Reference passed during exploration of different paths in the galaxy
#[derive(Clone)]
pub struct FalconState {
    pub planet: u32,
    pub cd: u32,
    pub path: Path,
    pub MAX_AUTONOMY: u32,
    pub seen: HashMap<(u32, u32), bool>,
}

impl FalconState {
    pub fn new(cd: u32, galaxy: &Galaxy, autonomy: u32, src_id: u32) -> FalconState {
        let mut seen = HashMap::new();
        for id in galaxy.planets.keys() {
            for count in 0..cd + 1 {
                seen.insert((id.clone(), count), false);
            }
        }
        FalconState {
            planet: src_id,
            cd: cd,
            path: Path::new(),
            MAX_AUTONOMY: autonomy,
            seen,
        }
    }
}

impl fmt::Debug for FalconState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nFalcon just lands a new planet:\n\tvisiting planet: {:?}\n\tcd: {:?}\n\tthrough path: {:?}\n", self.planet, self.cd, self.path)?;
        Ok(())
    }
}

fn display_seen(seen: &HashMap<(u32, u32), bool>) {
    let mut seens = String::from("");
    for ((i, j), b) in seen {
        if *b {
            seens.push_str(&format!("{:?}[{:?}], ", i, j));
        }
    }
    debug!("\t\tSeen Planets are: {:?}", seens);
}

/// Most important method
/// Visit the galaxy with a DFS based method with backtrack
/// It takes into account of autonomy, jump distances and times
pub fn generate_paths(
    galaxy: &Galaxy,
    src: String,
    dst: String,
    cd: u32,
    autonomy: u32,
) -> Vec<Path> {
    let mut all_paths: Vec<Path> = Vec::new();

    fn visit(
        src: u32,
        dst: u32,
        galaxy: &Galaxy,
        state: &mut FalconState,
        all_paths: &mut Vec<Path>,
        refuel: bool,
        autonomy: u32,
        time: u32,
    ) {
        state.seen.entry((src, time)).and_modify(|e| *e = true);
        state.path.add_planet_time(src, time);
        state.planet = src;

        debug!("{:?}", state);

        if src == dst {
            all_paths.push(state.path.clone());
        } else {
            if time < state.cd {
                for (planet, distance) in galaxy.planets.get(&src).unwrap().neighbors.iter() {
                    debug!("\tCandidate planet: {:?}", *planet);
                    debug!("\t\tLocated at distance: {:?}", distance);
                    debug!("\t\tRemaining autonomy: {:?}", autonomy);
                    debug!(
                        "\t\tNeighbors: {:?}",
                        galaxy.planets.get(planet).unwrap().neighbors
                    );
                    display_seen(&state.seen);
                    debug!("\t\tPath: {:?}", state.path);
                    debug!("\t\tIs distance <= autonomy: {:?}", *distance <= autonomy);

                    match state.seen.get(&(*planet, time + distance)) {
                        None => {
                            debug!("\t\tSeen or too far given the countdown!");
                            ()
                        }
                        Some(s) => {
                            debug!(
                                "\t\tSeen or not ?: {:?}",
                                if *s { "Seen" } else { "Not seen" }
                            );
                            if !s {
                                if *planet == src {
                                    visit(
                                        *planet,
                                        dst,
                                        galaxy,
                                        state,
                                        all_paths,
                                        true,
                                        state.MAX_AUTONOMY,
                                        time + 1,
                                    );
                                } else if *distance <= autonomy {
                                    visit(
                                        *planet,
                                        dst,
                                        galaxy,
                                        state,
                                        all_paths,
                                        false,
                                        autonomy - distance,
                                        time + distance,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        debug!("\nBACKTRACK from:\n\tplanet: {:?}\n\tautonomy: {:?}\n\trefuel: {:?}\n\tcd: {:?}\n\tat time: {:?}\n\tthrough path: {:?}\n", state.planet, autonomy, refuel, state.cd, time, state.path);
        state.path.planets.pop();
        state.seen.entry((src, time)).and_modify(|e| *e = false);
    };

    let src_id = galaxy.id.get(&src).unwrap();
    let dst_id = galaxy.id.get(&dst).unwrap();

    let mut state = FalconState::new(cd, galaxy, autonomy, *src_id);

    debug!("\nSeen Planets are: {:?}", state.seen);
    visit(
        *src_id,
        *dst_id,
        galaxy,
        &mut state,
        &mut all_paths,
        false,
        autonomy,
        0,
    );

    all_paths
}
