use std::fmt;
use std::collections::HashMap;

/// Nothing special here: just simple stuff related to the Galaxy management
///     - creates a directed graph
///     - add "virtual" edge on planets to simulate "refule" and "wait" action

pub fn from_routes<I>(routes: I) -> Galaxy
where
    I: IntoIterator<Item = (String, String, u32)>,
{
    let mut galaxy = Galaxy::new();
    for (src, dst, weight) in routes {
        galaxy.add_route(src, dst, weight);
    }
    galaxy
}

pub struct Planet {
    pub name: String,
    pub neighbors: Vec<(u32,u32)>,
}

impl Planet {
    pub fn new(name: String, id: u32) -> Planet {
        Planet { name: name, neighbors: vec![(id,1)] }
    }
}

impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ {} -> ", self.name)?;
        for &pl in self.neighbors.iter() {
            write!(f, " ({},{}) ", pl.0, pl.1)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

pub struct Galaxy {
    pub planets: HashMap<u32, Planet>,
    pub id: HashMap<String,u32>,
    pub idx: u32,
}

impl fmt::Debug for Galaxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (id,pl) in self.planets.iter() {
            write!(f, "\n{:?}:{:?}", id, pl)?;
        }
        Ok(())
    }
}

impl Galaxy {
    pub fn new() -> Galaxy {
        Galaxy { idx: 0, id: HashMap::new(), planets: HashMap::new() }
    }

    fn add_planet(&mut self, name: String) -> u32 {
        let planet_id = self.idx;
        self.id.insert(name.clone(), planet_id);
        self.planets.insert(planet_id, Planet::new(name, planet_id));
        self.idx += 1;
        planet_id
    }

    pub fn get_planet_id(&mut self, planet_name: String) -> u32 {
        for (id, planet) in self.planets.iter() {
            if planet.name == planet_name {
                return *id
            }
        }
        self.add_planet(planet_name)
    }

    fn add_edge(&mut self, src_id: u32, dst_id: u32, weight: u32) {
        self.planets.entry(src_id).and_modify(|p| { p.neighbors.push((dst_id,weight)) });
    }

    fn add_route(&mut self, src: String, dst: String, weight: u32) {
        let src_id = self.get_planet_id(src.clone());
        let dst_id = self.get_planet_id(dst.clone());

        self.add_edge(src_id, dst_id, weight);
        self.add_edge(dst_id, src_id, weight);
    }
}
