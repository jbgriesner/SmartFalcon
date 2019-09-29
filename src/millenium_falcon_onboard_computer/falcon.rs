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

fn init_seen_planets(galaxy: &Galaxy, cd: u32) -> HashMap<(u32,u32),bool> {
    let mut seen = HashMap::new();
    for id in galaxy.planets.keys() {
        for count in 0..cd {
            seen.insert((id.clone(),count), false);
        }
    }
    seen
}

pub fn generate_paths(galaxy: &Galaxy, src: String, dst: String, cd: u32, autonomy: u32) -> Vec<Path> {
    let mut all_paths: Vec<Path> = Vec::new();

    fn visit(time: u32, src: u32, dst: u32, galaxy: &Galaxy, path: &mut Path, all_paths: &mut Vec<Path>, seen: &mut HashMap<(u32,u32),bool>, cd: u32, autonomy: u32, dist: u32, MAX_AUTONOMY: u32) {
        println!("\nFalcon (autonomy: {}) visiting planet: {}, cd: {}, at time: {}, through path: {:?}", autonomy, src, cd, time, path);
        seen.entry((src,time)).and_modify(|e| { *e = true });
        path.add_planet_time(src, time);

        let time = time + dist;

        if time <= cd {
            if src == dst {
                all_paths.push(path.clone());
            } else {
                for (planet, distance) in galaxy.planets.get(&src).unwrap().neighbors.iter() {

                    println!("\tcandidate planet: {:?}, distance: {:?}", *planet, distance);
                    println!("\tneighbors: {:?}", galaxy.planets.get(planet).unwrap().neighbors);
                    //println!("\tseen_planets: {:?}", seen);
                    println!("\tCOND1: {:?}", seen.get(&(*planet,time)).unwrap());
                    println!("\tCOND2: {:?}", *distance < autonomy);

                    //println!("seen: {:?} planet: {}, time: {}, cd: {}", seen, *planet, time, cd);
                    if !seen.get(&(*planet,time)).unwrap() && *distance <= autonomy {
                        if *planet == src {
                            visit(time, *planet, dst, galaxy, path, all_paths, seen, cd, MAX_AUTONOMY, *distance, MAX_AUTONOMY);
                        } else {
                            visit(time, *planet, dst, galaxy, path, all_paths, seen, cd, autonomy - distance, *distance, MAX_AUTONOMY);
                        }
                    }
                }
            }
        }
        path.planets.pop();
        seen.entry((src,time)).and_modify(|e| { *e = false });
    };

    let time = 0;
    let MAX_AUTONOMY: u32 = autonomy;

    let mut seen_planets = init_seen_planets(galaxy, cd+1);

    let src_id = galaxy.id.get(&src).unwrap();
    let dst_id = galaxy.id.get(&dst).unwrap();

    let mut path = Path::new();

    visit(time, *src_id, *dst_id, galaxy, &mut path, &mut all_paths, &mut seen_planets, cd, autonomy, 1, MAX_AUTONOMY);

    all_paths
}

