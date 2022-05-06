use std::{collections::HashMap, fs::File, io::Read, path::Path};

use egui::Color32;
use serde::{Deserialize, Serialize};

const COLORS: [Color32; 4] = [
    // these are the colours that will be displayed
    Color32::RED,
    Color32::GREEN,
    Color32::BLUE,
    Color32::from_rgb(255, 0, 255),
];

#[derive(Clone, Serialize, Deserialize)]
pub struct Map(pub Vec<Cell>); // a map which is a wrapper for a list of cells

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cell {
    pub name: String,            // the name of the region
    pub connections: Vec<usize>, // the neighbors/connections of the region
    pub color: Option<usize>,    // the color of the region (if any)
}

impl Default for Map {
    fn default() -> Map {
        Map(Vec::new()) // create a default map
    }
}

impl Map {
    pub fn add_names(&mut self, names: Vec<String>) {
        // add various cells given their names.
        for name in names {
            self.add_cell(name)
        }
    }
    pub fn add_cell(&mut self, name: String) {
        self.0.push(Cell::new(name)) // add a cell to the map, given its name
    }
    pub fn color_map(&mut self) -> (bool, usize) {
        // start coloring the map
        // returns false if the map was failed to be colored.
        let start = self.0[0].clone();
        let mut x = 0;
        let res = start.color_in(0, self, &mut x);
        (res, x)
    }
    pub fn validate(&self) -> bool {
        // returns true if all connections are valid
        for (i, cell) in self.0.iter().enumerate() {
            for conn in &cell.connections {
                if !self.0[*conn].connections.contains(&i) {
                    println!("{} {:?} {}", i, cell, conn);
                    return false;
                }
            }
        }
        true
    }
    pub fn from_file(file: &Path) -> Map {
        // create a map from a file path
        let mut s = String::new();
        File::open(file)
            .unwrap()
            .read_to_string(&mut s)
            .expect("Reading file failed!");

        // parse file contents into a hashmap
        let v: HashMap<String, Vec<String>> =
            toml::from_str(s.as_str()).expect("Could not parse file value!");
        match v.try_into() {
            // try converting it into a map
            Ok(x) => x,                    // return the valid map
            Err(err) => panic!("{}", err), // panic if it cant be converted
        }
    }
}

impl Cell {
    pub fn new(name: String) -> Cell {
        // creates a default cell given a name
        Cell {
            name,
            connections: Vec::new(),
            color: None,
        }
    }
    pub fn link_changed(&mut self, other: usize) {
        // toggle a cells connection/link
        for (i, c) in self.connections.iter().enumerate() {
            if c == &other {
                // it was found we have to remove it
                self.connections.remove(i);
                return;
            }
        }
        // if it wasnt found we have to add it
        self.connections.push(other);
    }
    pub fn color(&self) -> Color32 {
        // give the actuall color or a default color
        if let Some(c) = self.color {
            COLORS[c] // actual color
        } else {
            Color32::BLACK // default color
        }
    }
    pub fn color_in(&self, i: usize, map: &mut Map, count: &mut usize) -> bool {
        // i is the position in the map of the current item
        *count += 1;

        if self.color.is_some() {
            // if its already colored, we continue
            return true;
        }
        // get all avalible colors for the current cell.
        let mut avalible = self.get_avalible(map);
        // try every avalible color to see if they work
        while !avalible.is_empty() {
            map.0[i].color = avalible.pop(); // gets the next color
                                             // get all connected cells and their index
            let connected_cells: Vec<(Cell, usize)> = self
                .connections
                .iter()
                .map(|n| (map.clone().0[*n].clone(), *n))
                .collect();

            // a bool keeping track if any of the calls failed
            let mut fail = false;

            for (cell, j) in connected_cells {
                // iterate through the neighbors
                if !cell.color_in(j, map, count) {
                    // recursively call the own functions on neighbors
                    fail = true; // if it fails we break
                    break;
                }
            }
            if !fail {
                return true; // if it didnt fail we return true, meaning the current one was successfully colored
            }
            // if not, we continue the loop with the next item, or we return false if there are no more
        }
        // reset the current cells color (as it clearly didnt work)
        map.0[i].color = None;
        false // return false since there were no correct countries
    }
    pub fn get_avalible(&self, map: &mut Map) -> Vec<usize> {
        // gets avalible colors for the country
        // all by default
        let mut avalible: Vec<usize> = (0..4).collect();
        // go through connections
        for n in self.connections.iter().map(|n| map.clone().0[*n].clone()) {
            if let Some(c) = n.color {
                // if they have a color
                let mut rm = None;

                // remove it
                for (j, col) in avalible.iter().enumerate() {
                    if &c == col {
                        rm = Some(j);
                    }
                }
                if let Some(r) = rm {
                    avalible.remove(r);
                }
            }
        }
        avalible // return all avalible colors
    }
}

impl From<Map> for HashMap<String, Vec<String>> {
    fn from(map: Map) -> HashMap<String, Vec<String>> {
        // convert this map into a hashmap of cells and their neighbors
        let mut h = HashMap::new();
        for cell in map.0.iter() {
            let mut v = Vec::new();
            for c in cell.connections.iter() {
                v.push(map.0[*c].name.clone());
            }
            h.insert(cell.name.clone(), v);
        }
        h
    }
}

impl TryFrom<HashMap<String, Vec<String>>> for Map {
    type Error = &'static str; // if it fails we have a static string being the error
    fn try_from(inp: HashMap<String, Vec<String>>) -> Result<Map, Self::Error> {
        let mut map = Map::default();
        map.add_names(inp.keys().cloned().collect());
        let mut cons: HashMap<usize, Vec<usize>> = HashMap::new();
        for (k, cell) in map.0.iter().enumerate() {
            if let Some(connections) = inp.get(&cell.name) {
                // Do some things
                let mut v = Vec::new();

                // iterate through connections
                for connection in connections {
                    if let Some(ind) = map.0.iter().enumerate().find_map(|(i, cell2)| {
                        if &cell2.name == connection {
                            Some(i)
                        } else {
                            None
                        }
                    }) {
                        v.push(ind)
                    } else {
                        println!("{} {}", cell.name, connection);
                        return Err("The input hashmap was invalid (Could not find neighbor).");
                    }
                }
                cons.insert(k, v);
            } else {
                return Err("The input hashmap was invalid.");
            }
        }
        for (i, cons) in cons {
            map.0[i].connections = cons;
        }
        // quick check to ensure that the map is correct
        if !map.validate() {
            return Err("The input hashmap was invalid. (Validation Failed)");
        }
        Ok(map)
    }
}
