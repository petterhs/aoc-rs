use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    time::Instant,
    vec,
};

#[derive(Clone, Debug, PartialEq)]
struct Valve {
    name: String,
    flow_rate: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct Tunnel {
    to: String,
    time: i32,
}

impl Tunnel {
    fn new(to: &str, time: i32) -> Tunnel {
        Tunnel {
            to: to.to_string(),
            time,
        }
    }
}

#[derive(Debug)]
struct Graph {
    valves: HashMap<String, Valve>,
    tunnels: HashMap<String, Vec<Tunnel>>,
    valve_index: HashMap<String, usize>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for (valve, valve_info) in &self.valves {
            s.push_str(&format!("{} {}\n", valve, valve_info.flow_rate));
        }

        for (valve, tunnels) in &self.tunnels {
            s.push_str(&format!("{}: ", valve));
            for tunnel in tunnels {
                s.push_str(&format!("{} ", tunnel.to));
            }
            s.push_str("\n");
        }

        write!(f, "{}", s)
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {
            valves: HashMap::new(),
            tunnels: HashMap::new(),
            valve_index: HashMap::new(),
        }
    }

    fn add_valve(&mut self, name: &str, flow_rate: i32) {
        self.valves.insert(
            name.to_string(),
            Valve {
                name: name.to_string(),
                flow_rate,
            },
        );
    }

    fn add_tunnels(&mut self, from: &str, to: Vec<String>, time: i32) {
        if !self.tunnels.contains_key(from) {
            self.tunnels.insert(from.to_string(), vec![]);
        }

        for tunnel in to {
            self.tunnels
                .get_mut(from)
                .unwrap()
                .push(Tunnel::new(&tunnel, time));
        }
    }

    fn parse_line(&mut self, line: &str) {
        let s = line.split("; ").collect::<Vec<&str>>();

        let valve = s[0].split(" has flow rate=").collect::<Vec<&str>>();
        let name = valve[0].split("Valve ").collect::<Vec<&str>>()[1];
        let flow_rate = valve[1].parse::<usize>().unwrap();

        self.add_valve(name, flow_rate as i32);

        let tunnels = s[1]
            .split_whitespace()
            .skip(4)
            .collect::<Vec<&str>>()
            .join("")
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        self.add_tunnels(name, tunnels, 1);
    }

    fn optimize_graph(&mut self) -> (usize, Vec<usize>, Vec<Vec<usize>>) {
        let mut index = HashMap::new();
        let mut v = Vec::new();
        let mut start_index = 0;
        for (_, valve) in self.valves.iter() {
            if valve.flow_rate != 0 || valve.name == "AA" {
                v.push(valve.flow_rate as usize);
                index.insert(valve.name.clone(), v.len() - 1);
            }
            if valve.name == "AA" {
                start_index = v.len() - 1;
            }
        }

        let mut tunnels = HashMap::new();
        for valve in self.valves.keys() {
            tunnels.insert(valve.clone(), vec![]);
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back((valve.to_string(), 0));

            while let Some((other_valve, time)) = queue.pop_front() {
                if visited.contains(&other_valve) {
                    continue;
                }

                if other_valve != *valve && self.valves[&other_valve].flow_rate != 0 {
                    tunnels
                        .get_mut(valve.as_str())
                        .unwrap()
                        .push(Tunnel::new(&other_valve, time));
                }

                visited.insert(other_valve.clone());

                for tunnel in self.tunnels.get(&other_valve).unwrap() {
                    if visited.contains(&tunnel.to) {
                        continue;
                    }

                    queue.push_back((tunnel.to.clone(), time + tunnel.time));
                }
            }
        }

        tunnels = tunnels
            .into_iter()
            .filter(|(valve, _)| self.valves.contains_key(valve))
            .collect::<HashMap<String, Vec<Tunnel>>>();

        let mut t = vec![vec![0; v.len()]; v.len()];
        for valve in index.keys() {
            for tunnel in tunnels.get(valve).unwrap() {
                if tunnel.to == "AA" {
                    continue;
                }
                t[index[valve]][index[&tunnel.to]] = tunnel.time as usize;
            }
        }
        // println!("start: {}", start_index);
        // println!("v: {:?}", v);
        // println!("t: {:?}", t);
        self.valve_index = index;
        // println!("neigbour_shortest_dist: {:?}", neigbour_shortest_dist);
        return (start_index, v, t);
    }

    fn dfs(
        &self,
        valve_index: usize,
        valves: &Vec<usize>,
        neigbour_shortest_dist: &Vec<Vec<usize>>,
        time_left: i32,
        opened_valves_bitmask: u32,
        cache: &mut HashMap<(usize, i32, u32), i32>,
    ) -> i32 {
        if let Some(&max) = cache.get(&(valve_index, time_left, opened_valves_bitmask)) {
            return max;
        }

        let mut max = 0;
        for (i, time) in neigbour_shortest_dist[valve_index].iter().enumerate() {
            if opened_valves_bitmask & (1 << i) != 0 || valves[i] == 0 {
                continue;
            }

            let time_left = time_left - *time as i32 - 1;

            if time_left <= 0 {
                continue;
            }

            let pressure_released = valves[i] as i32 * time_left;

            max = max.max(
                self.dfs(
                    i,
                    valves,
                    neigbour_shortest_dist,
                    time_left,
                    opened_valves_bitmask | 1 << i,
                    cache,
                ) + pressure_released,
            );
        }
        cache.insert((valve_index, time_left, opened_valves_bitmask), max);
        return max;
    }
}

fn part1(input: &str) -> i32 {
    let mut graph = Graph::new();

    for line in input.lines() {
        graph.parse_line(line);
    }

    let start = Instant::now();
    let (start_index, valves, neigbour_shortest_dist) = graph.optimize_graph();
    println!("Time: {}", start.elapsed().as_millis());

    let mut cache = HashMap::new();

    let max = graph.dfs(
        start_index,
        &valves,
        &neigbour_shortest_dist,
        30,
        1 << start_index,
        &mut cache,
    );
    println!("Time: {}", start.elapsed().as_millis());

    let (valve_index, time_left, max_opened_valves_bitmask) = cache.iter().max().unwrap().0;
    println!("opened_valves_bitmask: {:b}", max_opened_valves_bitmask);
    max
}

fn part2(input: &str) -> i32 {
    let mut graph = Graph::new();

    for line in input.lines() {
        graph.parse_line(line);
    }

    let (start_index, valves, neighbours_shortest_dist) = graph.optimize_graph();

    //find the maximum number of different bitmasks
    let num_valves = valves.len();
    println!("num_valves: {}", num_valves);

    let num_bitmasks = 2_u32.pow(num_valves as u32);
    println!("num_bitmasks: {}", num_bitmasks);

    let mut max = 0;

    let mut cache = HashMap::new();
    // let mut cache2 = HashMap::new();
    let _ = graph.dfs(
        start_index,
        &valves,
        &neighbours_shortest_dist,
        26,
        0,
        &mut cache,
    );
    let mut cache2 = HashMap::new();
    // let mut cache2 = HashMap::new();
    let _ = graph.dfs(
        start_index,
        &valves,
        &neighbours_shortest_dist,
        26,
        0,
        &mut cache2,
    );

    let aa_index = graph.valve_index.get("AA").unwrap();

    // let mut cache2 = cache.clone();
    for ((_, _, bitmask), my_pressure_released) in &cache {
        for ((_, _, bitmask2), elephant_pressure_released) in &cache {
            if bitmask & bitmask2 == 0 {
                println!("bitmask: {:b}", bitmask);
                println!("bitmask2: {:b}", bitmask2);
                println!("AND: {:b}", bitmask & bitmask2);
                println!(
                    "Released: {}",
                    my_pressure_released + elephant_pressure_released
                );

                max = max.max(my_pressure_released + elephant_pressure_released);
            }
        }
    }
    println!("aa_index: {}: {:b}", aa_index, 1 << aa_index);

    max
}

pub fn run() {
    println!("Day 16");
    let input = include_str!("../input/test16");

    let start = Instant::now();
    println!("Part1: {}", part1(input));
    println!("Time: {}", start.elapsed().as_millis());

    let start = Instant::now();
    println!("Part2: {}", part2(input));
    println!("Time: {}", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let input = "Valve AA has flow rate=1; tunnels lead to valves BB, CC, DD";

        let mut graph = Graph::new();
        graph.parse_line(input);

        assert_eq!(graph.valves.len(), 1);
    }

    #[test]
    fn parse_optimize_graph() {
        let mut graph = Graph::new();
        let input = include_str!("../input/test16");

        for line in input.lines() {
            graph.parse_line(line);
        }

        let (_start_index, _valves, neighbours) = graph.optimize_graph();

        let aa_index = graph.valve_index.get("AA").unwrap();
        let bb_index = graph.valve_index.get("BB").unwrap();
        let cc_index = graph.valve_index.get("CC").unwrap();
        let dd_index = graph.valve_index.get("DD").unwrap();
        let ee_index = graph.valve_index.get("EE").unwrap();
        let hh_index = graph.valve_index.get("HH").unwrap();
        let jj_index = graph.valve_index.get("JJ").unwrap();

        let aa_neighbours = neighbours[*aa_index].clone();
        assert_eq!(aa_neighbours[*bb_index], 1);
        assert_eq!(aa_neighbours[*cc_index], 2);
        assert_eq!(aa_neighbours[*dd_index], 1);
        assert_eq!(aa_neighbours[*ee_index], 2);
        assert_eq!(aa_neighbours[*hh_index], 5);
        assert_eq!(aa_neighbours[*jj_index], 2);

        let bb_neighbours = neighbours[*bb_index].clone();
        assert_eq!(bb_neighbours[*aa_index], 0);
        assert_eq!(bb_neighbours[*cc_index], 1);
        assert_eq!(bb_neighbours[*dd_index], 2);
        assert_eq!(bb_neighbours[*ee_index], 3);
        assert_eq!(bb_neighbours[*hh_index], 6);
        assert_eq!(bb_neighbours[*jj_index], 3);

        let cc_neighbours = neighbours[*cc_index].clone();
        assert_eq!(cc_neighbours[*aa_index], 0);
        assert_eq!(cc_neighbours[*bb_index], 1);
        assert_eq!(cc_neighbours[*dd_index], 1);
        assert_eq!(cc_neighbours[*ee_index], 2);
        assert_eq!(cc_neighbours[*hh_index], 5);
        assert_eq!(cc_neighbours[*jj_index], 4);
        assert_eq!(cc_neighbours[*jj_index], 4);

        let dd_neighbours = neighbours[*dd_index].clone();
        assert_eq!(dd_neighbours[*aa_index], 0);
        assert_eq!(dd_neighbours[*bb_index], 2);
        assert_eq!(dd_neighbours[*cc_index], 1);
        assert_eq!(dd_neighbours[*ee_index], 1);
        assert_eq!(dd_neighbours[*hh_index], 4);
        assert_eq!(dd_neighbours[*jj_index], 3);

        let ee_neighbours = neighbours[*ee_index].clone();
        assert_eq!(ee_neighbours[*aa_index], 0);
        assert_eq!(ee_neighbours[*bb_index], 3);
        assert_eq!(ee_neighbours[*cc_index], 2);
        assert_eq!(ee_neighbours[*dd_index], 1);
        assert_eq!(ee_neighbours[*hh_index], 3);
        assert_eq!(ee_neighbours[*jj_index], 4);

        let hh_neighbours = neighbours[*hh_index].clone();
        assert_eq!(hh_neighbours[*aa_index], 0);
        assert_eq!(hh_neighbours[*bb_index], 6);
        assert_eq!(hh_neighbours[*cc_index], 5);
        assert_eq!(hh_neighbours[*dd_index], 4);
        assert_eq!(hh_neighbours[*ee_index], 3);
        assert_eq!(hh_neighbours[*jj_index], 7);

        let jj_neighbours = neighbours[*jj_index].clone();
        assert_eq!(jj_neighbours[*aa_index], 0);
        assert_eq!(jj_neighbours[*bb_index], 3);
        assert_eq!(jj_neighbours[*cc_index], 4);
        assert_eq!(jj_neighbours[*dd_index], 3);
        assert_eq!(jj_neighbours[*ee_index], 4);
        assert_eq!(jj_neighbours[*hh_index], 7);
    }

    // #[test]
    fn test_part1() {
        let input = include_str!("../input/test16");

        assert_eq!(part1(input), 1651);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/test16");

        assert_eq!(part2(input), 1707);
    }
}
