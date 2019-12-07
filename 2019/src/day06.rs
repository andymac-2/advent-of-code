use std::io;
use std::io::Read;

use std::collections::HashMap;

use nom::character::complete::{alphanumeric1, char, newline};
use nom::combinator::map;
use nom::multi::separated_list;
use nom::sequence::tuple;
use nom::IResult;

struct Planet<'str> {
    parent_name: &'str str,
}

pub struct PlanetTree<'str> {
    map: HashMap<&'str str, Planet<'str>>,
}
impl<'str> PlanetTree<'str> {
    pub fn from_string(string: &'str str) -> Self {
        let mut map = HashMap::new();
        for (parent, child) in Self::edges_p(string).unwrap().1 {
            let planet = Planet {
                parent_name: parent,
            };
            if map.insert(child, planet).is_some() {
                panic!("Not a tree");
            }
        }
        PlanetTree { map }
    }
    pub fn get_parent(&self, planet_name: &'str str) -> Option<&'str str> {
        self.map.get(planet_name).map(|planet| planet.parent_name)
    }
    pub fn get_parents(&self, mut planet_name: &'str str) -> Vec<&'str str> {
        let mut vec = Vec::new();
        loop {
            vec.push(planet_name);
            if let Some(parent_name) = self.get_parent(planet_name) {
                planet_name = parent_name;
            } else {
                break;
            }
        }
        vec
    }
    pub fn get_distance(&self, planet1: &'str str, planet2: &'str str) -> usize {
        let mut parents1 = self.get_parents(planet1);
        let mut parents2 = self.get_parents(planet2);

        // find lowest common ancestor
        while parents1.last() == parents2.last() {
            parents1.pop();
            parents2.pop();
        }

        parents1.len() + parents2.len()
    }

    // Parsers
    fn edge_p(input: &str) -> IResult<&str, (&str, &str)> {
        map(
            tuple((alphanumeric1, char(')'), alphanumeric1)),
            |(parent, _, child)| (parent, child),
        )(input)
    }
    fn edges_p(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
        separated_list(newline, Self::edge_p)(input)
    }
}

struct PlanetTreeView<'ptr, 'str> {
    tree: &'ptr PlanetTree<'str>,
    orbits: HashMap<&'str str, Option<u32>>,
}
impl<'ptr, 'str> PlanetTreeView<'ptr, 'str> {
    fn new(tree: &'ptr PlanetTree<'str>) -> Self {
        PlanetTreeView {
            tree,
            orbits: HashMap::new(),
        }
    }
    fn get_orbit_count(&mut self, planet_name: &'str str) -> Option<u32> {
        if let Some(result) = self.orbits.get(planet_name) {
            return *result;
        }
        self.orbits.insert(planet_name, None);

        let result = self
            .tree
            .get_parent(planet_name)
            .map_or(Some(0), |parent_name| {
                Some(self.get_orbit_count(parent_name)? + 1)
            });

        self.orbits.insert(planet_name, result);
        result
    }
    fn get_orbit_counts(&mut self) -> u32 {
        self.tree
            .map
            .keys()
            .filter_map(|planet| self.get_orbit_count(planet))
            .sum()
    }
}

pub fn part1(tree: &PlanetTree) -> u32 {
    let mut view = PlanetTreeView::new(tree);
    view.get_orbit_counts()
}

pub fn part2(tree: &PlanetTree) -> usize {
    tree.get_distance("YOU", "SAN").saturating_sub(2)
}

pub fn start() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let tree = PlanetTree::from_string(&buffer);

    println!("Program Output: {:?}", part2(&tree));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_orbits() {
        let tree =
            PlanetTree::from_string("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L");
        assert_eq!(part1(&tree), 42);
    }
    #[test]
    fn example_distance() {
        let tree = PlanetTree::from_string(
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
        );
        assert_eq!(part2(&tree), 4);
    }
}
