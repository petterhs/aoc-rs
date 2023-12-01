use std::{
    ops::AddAssign,
    ops::{Add, Sub},
    str::FromStr,
};

struct Blueprint {
    id: usize,
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: (usize, usize),
    geode_cost: (usize, usize),
    max_ore_robots: usize,
    max_clay_robots: usize,
}

impl FromStr for Blueprint {
    //Example input:
    //Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //Filter away the non-numeric characters
        //Remove the colon
        let mut words = s
            .split_whitespace()
            .map(|c| c.trim_matches(|c| c == ':'))
            .filter(|c| c.chars().all(char::is_numeric))
            .map(|c| c.parse().unwrap());

        let b = Blueprint {
            id: words.next().unwrap(),
            ore_cost: words.next().unwrap(),
            clay_cost: words.next().unwrap(),
            obsidian_cost: (words.next().unwrap(), words.next().unwrap()),
            geode_cost: (words.next().unwrap(), words.next().unwrap()),
            max_ore_robots: 0,
            max_clay_robots: 0,
        };

        let max_ore_robots = std::cmp::max(
            b.ore_cost,
            std::cmp::max(
                b.clay_cost,
                std::cmp::max(b.obsidian_cost.0, b.geode_cost.0),
            ),
        );

        let max_clay_robots = std::cmp::max(b.clay_cost, b.obsidian_cost.1);

        Ok(Blueprint {
            max_ore_robots,
            max_clay_robots,
            ..b
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Inventory {
    fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        Inventory {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

//Implement add for inventory

impl AddAssign for Inventory {
    fn add_assign(&mut self, other: Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}

impl Add<Inventory> for Inventory {
    type Output = Inventory;

    fn add(self, other: Inventory) -> Inventory {
        Inventory {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub<Inventory> for Inventory {
    type Output = Inventory;

    fn sub(self, other: Inventory) -> Inventory {
        Inventory {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Blueprint {
    fn max_geode(
        &self,
        time: usize,
        robots: Inventory,
        inventory: Inventory,
        after_empty: Option<Vec<usize>>,
        current_max: usize,
    ) -> usize {
        if time == 1 {
            return inventory.geode + robots.geode;
        }

        if potential_material(time, inventory.geode, robots.geode) <= current_max {
            return 0;
        }

        if potential_material(time, inventory.obsidian, robots.obsidian) <= self.geode_cost.1 {
            return inventory.geode + robots.geode * time;
        }

        if inventory.ore >= self.geode_cost.0 && inventory.obsidian >= self.geode_cost.1 {
            return self.max_geode(
                time - 1,
                robots + Inventory::new(0, 0, 0, 1),
                inventory + robots - Inventory::new(self.geode_cost.0, 0, self.geode_cost.1, 0),
                None,
                current_max,
            );
        }

        let mut geodes = current_max;

        let after_empty = after_empty.unwrap_or(vec![0; 3]);

        let mut could_build = vec![0; 3];
        if inventory.ore >= self.obsidian_cost.0
            && inventory.clay >= self.obsidian_cost.1
            && robots.obsidian < self.geode_cost.1
            && after_empty[0] == 0
        {
            geodes = geodes.max(self.max_geode(
                time - 1,
                robots + Inventory::new(0, 0, 1, 0),
                inventory + robots
                    - Inventory::new(self.obsidian_cost.0, self.obsidian_cost.1, 0, 0),
                None,
                geodes,
            ));
            could_build[0] = 1;
        }

        if inventory.ore >= self.clay_cost
            && robots.clay < self.max_clay_robots
            && after_empty[2] == 0
        {
            geodes = geodes.max(self.max_geode(
                time - 1,
                robots + Inventory::new(0, 1, 0, 0),
                inventory + robots - Inventory::new(self.clay_cost, 0, 0, 0),
                None,
                geodes,
            ));
            could_build[2] = 1;
        }

        if inventory.ore >= self.ore_cost && robots.ore < self.max_ore_robots && after_empty[1] == 0
        {
            geodes = geodes.max(self.max_geode(
                time - 1,
                robots + Inventory::new(1, 0, 0, 0),
                inventory + robots - Inventory::new(self.ore_cost, 0, 0, 0),
                None,
                geodes,
            ));
            could_build[1] = 1;
        }

        geodes = geodes.max(self.max_geode(
            time - 1,
            robots,
            inventory + robots,
            Some(could_build),
            geodes,
        ));
        return geodes;
    }
}

fn part1(input: &str) -> usize {
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let blueprint = Blueprint::from_str(line).unwrap();
        blueprints.push(blueprint);
    }

    let mut max_geodes = Vec::new();
    let mut sum_quality = 0;

    for blueprint in blueprints {
        let start = std::time::Instant::now();
        max_geodes.push(blueprint.max_geode(
            24,
            Inventory::new(1, 0, 0, 0),
            Inventory::new(0, 0, 0, 0),
            None,
            0,
        ));
        let duration = start.elapsed();
        println!(
            "Geodes: id: {}, max: {}, time: {:?}",
            blueprint.id,
            max_geodes.last().unwrap(),
            duration
        );
        sum_quality += blueprint.id * max_geodes.last().unwrap();
    }

    return sum_quality;
}
fn potential_material(time_left: usize, material_inventory: usize, robots: usize) -> usize {
    return material_inventory + time_left * robots + time_left * (time_left - 1) / 2;
}

fn part2(input: &str) -> usize {
    let mut blueprints = Vec::new();

    let mut lines = input.lines().take(3);

    for line in lines.by_ref() {
        let blueprint = Blueprint::from_str(line).unwrap();

        blueprints.push(blueprint);
    }
    let mut max_geodes = Vec::new();
    for blueprint in blueprints {
        //time function
        let start = std::time::Instant::now();
        max_geodes.push(blueprint.max_geode(
            32,
            Inventory::new(1, 0, 0, 0),
            Inventory::new(0, 0, 0, 0),
            None,
            0,
        ));
        let duration = start.elapsed();
        println!(
            "Geodes: id: {}, max: {}, time: {:?}",
            blueprint.id,
            max_geodes.last().unwrap(),
            duration
        );
    }
    max_geodes.iter().fold(1, |acc, x| acc * x)
}

pub fn run() {
    let input = include_str!("../input/19");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input/test19");

        let mut blueprints = Vec::new();

        for line in input.lines() {
            let blueprint = Blueprint::from_str(line).unwrap();
            blueprints.push(blueprint);
        }
        assert_eq!(blueprints.len(), 2);
        assert_eq!(blueprints[0].id, 1);
        assert_eq!(blueprints[0].ore_cost, 4);
        assert_eq!(blueprints[0].clay_cost, 2);
        assert_eq!(blueprints[0].obsidian_cost, (3, 14));
        assert_eq!(blueprints[0].geode_cost, (2, 7));

        assert_eq!(blueprints[1].id, 2);
        assert_eq!(blueprints[1].ore_cost, 2);
        assert_eq!(blueprints[1].clay_cost, 3);
        assert_eq!(blueprints[1].obsidian_cost, (3, 8));
        assert_eq!(blueprints[1].geode_cost, (3, 12));
    }
}
