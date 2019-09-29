use std::fmt;
use std::collections::HashMap;
use crate::millenium_falcon_onboard_computer::galaxy::Galaxy;

#[derive(Default,Clone)]
pub struct Path {
    pub planets: Vec<(u32,u32)>,
    pub odds: f64,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (pl,time) in self.planets.iter() {
            write!(f, "{:?}[{:?}] -> ", pl, time)?;
        }
        Ok(())
    }
}

impl Path {
    fn new() -> Path {
        Path { planets: Vec::new(), ..Default::default()}
    }

    fn add_planet_time(&mut self, planet_id: u32, time: u32) {
        self.planets.push((planet_id, time));
    }
}

#[derive(Clone)]
pub struct FalconState {
    pub planet: u32,
    pub cd: u32,
    pub path: Path,
    pub MAX_AUTONOMY: u32,
    pub seen: HashMap<(u32,u32),bool>,
}

impl FalconState {
    pub fn new(cd: u32, galaxy: &Galaxy, autonomy: u32, src_id: u32) -> FalconState {
        let mut seen = HashMap::new();
        for id in galaxy.planets.keys() {
            for count in 0..cd+1 {
                seen.insert((id.clone(),count), false);
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

fn display_seen(seen: &HashMap<(u32,u32),bool>) {
    let mut seens = String::from("");
    for ((i,j),b) in seen {
        if *b {
            seens.push_str(&format!("{:?}[{:?}], ", i, j));
        }
    }
    println!("\t\tSeen Planets are: {:?}", seens);
}

pub fn generate_paths(galaxy: &Galaxy, src: String, dst: String, cd: u32, autonomy: u32) -> Vec<Path> {
    let mut all_paths: Vec<Path> = Vec::new();

    fn visit(src: u32, dst: u32, galaxy: &Galaxy, state: &mut FalconState, all_paths: &mut Vec<Path>, refuel: bool, autonomy: u32, time: u32) {
        state.seen.entry((src, time)).and_modify(|e| { *e = true });
        state.path.add_planet_time(src, time);
        state.planet = src;

        println!("{:?}", state);

        if src == dst {
            all_paths.push(state.path.clone());
        } else {
            if time < state.cd {
                for (planet, distance) in galaxy.planets.get(&src).unwrap().neighbors.iter() {

                    println!("\tCandidate planet: {:?}", *planet);
                    println!("\t\tLocated at distance: {:?}", distance);
                    println!("\t\tRemaining autonomy: {:?}", autonomy);
                    println!("\t\tNeighbors: {:?}", galaxy.planets.get(planet).unwrap().neighbors);
                    display_seen(&state.seen);
                    println!("\t\tPath: {:?}", state.path);
                    println!("\t\tIs distance <= autonomy: {:?}", *distance <= autonomy);

                    match state.seen.get(&(*planet, time+distance)) {
                        None => {
                            println!("\t\tSeen or too far given the countdown!");
                            ()
                        },
                        Some(s) => {
                            println!("\t\tSeen or not ?: {:?}", if *s {"Seen"} else {"Not seen"});
                            if !s {
                                if *planet == src {
                                    visit(*planet, dst, galaxy, state, all_paths, true, state.MAX_AUTONOMY, time+1);
                                } else if *distance <= autonomy {
                                    //println!("\t\t\t\t\tAAAAAAAAAAAA {:?}, {:?}", autonomy, distance);
                                    visit(*planet, dst, galaxy, state, all_paths, false, autonomy-distance, time+distance);
                                }
                            }
                        },
                    }
                }
            }
        }
        println!("\nBACKTRACK from:\n\tplanet: {:?}\n\tautonomy: {:?}\n\trefuel: {:?}\n\tcd: {:?}\n\tat time: {:?}\n\tthrough path: {:?}\n", state.planet, autonomy, refuel, state.cd, time, state.path);
        state.path.planets.pop();
        state.seen.entry((src, time)).and_modify(|e| { *e = false });
    };

    let src_id = galaxy.id.get(&src).unwrap();
    let dst_id = galaxy.id.get(&dst).unwrap();

    let mut state = FalconState::new(cd, galaxy, autonomy, *src_id);

    println!("\nSeen Planets are: {:?}", state.seen);
    visit(*src_id, *dst_id, galaxy, &mut state, &mut all_paths, false, autonomy, 0);

    all_paths
}

