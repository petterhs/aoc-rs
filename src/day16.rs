use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    time::Instant,
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
    cache: HashMap<(String, i32, u32), i32>,
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
            cache: HashMap::new(),
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
        let flow_rate = valve[1].parse::<i32>().unwrap();

        self.add_valve(name, flow_rate);

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

    fn optimize_graph(&mut self) {
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

        self.valves
            .retain(|_, valve| valve.flow_rate != 0 || valve.name == "AA");
        tunnels = tunnels
            .into_iter()
            .filter(|(valve, _)| self.valves.contains_key(valve))
            .collect::<HashMap<String, Vec<Tunnel>>>();

        self.tunnels = tunnels;
    }

    fn dfs(
        &self,
        valve: String,
        time_left: i32,
        opened_valves_bitmask: u32,
        bitmask_index: &HashMap<String, usize>,
        cache: &mut HashMap<(String, i32, u32), i32>,
    ) -> i32 {
        if let Some(&max) = self
            .cache
            .get(&(valve.clone(), time_left, opened_valves_bitmask))
        {
            return max;
        }

        let mut max = 0;
        for tunnel in self.tunnels.get(&valve).unwrap() {
            if opened_valves_bitmask & (1 << bitmask_index[&tunnel.to]) != 0 {
                continue;
            }

            let time_left = time_left - tunnel.time - 1;

            if time_left <= 0 {
                continue;
            }

            let pressure_released = self.valves[&tunnel.to].flow_rate * time_left;

            max = max.max(
                self.dfs(
                    tunnel.to.clone(),
                    time_left,
                    opened_valves_bitmask | 1 << bitmask_index[&tunnel.to],
                    bitmask_index,
                    cache,
                ) + pressure_released,
            );
        }
        cache.insert((valve.clone(), time_left, opened_valves_bitmask), max);
        return max;
    }
}

fn part1() -> i32 {
    let mut graph = Graph::new();
    let input = include_str!("../input/16");

    for line in input.lines() {
        graph.parse_line(line);
    }

    graph.optimize_graph();

    let mut cache = HashMap::new();
    let mut bitmask_index = HashMap::new();

    for (i, valve) in graph.valves.keys().enumerate() {
        bitmask_index.insert(valve.clone(), i);
    }

    let max = graph.dfs("AA".to_string(), 30, 0, &bitmask_index, &mut cache);
    max
}

fn part2() -> i32 {
    let mut graph = Graph::new();
    let input = include_str!("../input/16");

    for line in input.lines() {
        graph.parse_line(line);
    }

    graph.optimize_graph();

    //find the maximum number of different bitmasks
    let num_valves = graph.valves.len();
    println!("num_valves: {}", num_valves);

    let num_bitmasks = 2_u32.pow(num_valves as u32);
    println!("num_bitmasks: {}", num_bitmasks);

    let mut max = 0;
    let mut bitmask_index = HashMap::new();

    for (i, valve) in graph.valves.keys().enumerate() {
        bitmask_index.insert(valve.clone(), i);
    }

    let mut cache = HashMap::new();
    // let mut cache2 = HashMap::new();

    for i in 0..num_bitmasks {
        let pressure_released = graph.dfs("AA".to_string(), 26, i, &bitmask_index, &mut cache)
            + graph.dfs("AA".to_string(), 26, !i, &bitmask_index, &mut cache);

        if pressure_released > max {
            println!("i: {}, pressure_released: {}", i, pressure_released);
            max = pressure_released;
        }
    }

    max
}

pub fn run() {
    println!("Day 16");

    let start = Instant::now();
    println!("Part1: {}", part1());
    println!("Time: {}", start.elapsed().as_millis());

    let start = Instant::now();
    println!("Part!: {}", part2());
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

        graph.optimize_graph();

        let mut tunnels_aa = graph.tunnels.get("AA").unwrap().clone();
        tunnels_aa.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnels_aa,
            vec![
                Tunnel::new("BB", 1),
                Tunnel::new("CC", 2),
                Tunnel::new("DD", 1),
                Tunnel::new("EE", 2),
                Tunnel::new("HH", 5),
                Tunnel::new("JJ", 2),
            ]
        );

        let mut tunnels_bb = graph.tunnels.get("BB").unwrap().clone();
        tunnels_bb.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnels_bb,
            vec![
                Tunnel::new("CC", 1),
                Tunnel::new("DD", 2),
                Tunnel::new("EE", 3),
                Tunnel::new("HH", 6),
                Tunnel::new("JJ", 3),
            ]
        );

        let mut tunnels_cc = graph.tunnels.get("CC").unwrap().clone();
        tunnels_cc.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnels_cc,
            vec![
                Tunnel::new("BB", 1),
                Tunnel::new("DD", 1),
                Tunnel::new("EE", 2),
                Tunnel::new("HH", 5),
                Tunnel::new("JJ", 4),
            ]
        );

        let mut tunnels_dd = graph.tunnels.get("DD").unwrap().clone();
        tunnels_dd.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnels_dd,
            vec![
                Tunnel::new("BB", 2),
                Tunnel::new("CC", 1),
                Tunnel::new("EE", 1),
                Tunnel::new("HH", 4),
                Tunnel::new("JJ", 3),
            ]
        );

        let mut tunnels_ee = graph.tunnels.get("EE").unwrap().clone();
        tunnels_ee.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnels_ee,
            vec![
                Tunnel::new("BB", 3),
                Tunnel::new("CC", 2),
                Tunnel::new("DD", 1),
                Tunnel::new("HH", 3),
                Tunnel::new("JJ", 4),
            ]
        );

        let mut tunnels_hh = graph.tunnels.get("HH").unwrap().clone();
        tunnels_hh.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnels_hh,
            vec![
                Tunnel::new("BB", 6),
                Tunnel::new("CC", 5),
                Tunnel::new("DD", 4),
                Tunnel::new("EE", 3),
                Tunnel::new("JJ", 7),
            ]
        );

        let mut tunnels_jj = graph.tunnels.get("JJ").unwrap().clone();
        tunnels_jj.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnels_jj,
            vec![
                Tunnel::new("BB", 3),
                Tunnel::new("CC", 4),
                Tunnel::new("DD", 3),
                Tunnel::new("EE", 4),
                Tunnel::new("HH", 7),
            ]
        );
    }

    #[test]
    fn test_bitmask() {
        let mut graph = Graph::new();
        let input = include_str!("../input/test16");

        for line in input.lines() {
            graph.parse_line(line);
        }

        graph.optimize_graph();

        let mut bitmask_index = HashMap::new();
        let mut bits = 0;

        for (i, valve) in graph.valves.keys().enumerate() {
            bitmask_index.insert(valve, i);
            bits += 1;
        }

        //where all valves are opened
        let mut max_bitmask = 0_u32;
        for i in 0..bits {
            max_bitmask |= 1 << i;
        }

        assert_eq!(max_bitmask, 0b1111111);

        let mut bitmask = 0;

        bitmask |= 1 << bitmask_index[&"AA".to_string()];

        assert_ne!(bitmask & (1 << bitmask_index[&"AA".to_string()]), 0);
        assert_eq!(bitmask & (1 << bitmask_index[&"BB".to_string()]), 0);

        bitmask |= 1 << bitmask_index[&"BB".to_string()];

        assert_ne!(bitmask & (1 << bitmask_index[&"AA".to_string()]), 0);
        assert_ne!(bitmask & (1 << bitmask_index[&"BB".to_string()]), 0);

        for valve in graph.valves.keys() {
            bitmask |= 1 << bitmask_index[valve];
        }

        assert_eq!(bitmask, max_bitmask);
    }

    #[test]
    fn test_part1() {
        let mut graph = Graph::new();
        let input = include_str!("../input/test16");

        for line in input.lines() {
            graph.parse_line(line);
        }

        graph.optimize_graph();

        let mut cache = HashMap::new();
        let mut bitmask_index = HashMap::new();

        for (i, valve) in graph.valves.keys().enumerate() {
            bitmask_index.insert(valve.clone(), i);
        }

        let max = graph.dfs("AA".to_string(), 30, 0, &bitmask_index, &mut cache);

        assert_eq!(max, 1651);
    }

    #[test]
    fn test_part2() {
        let mut graph = Graph::new();
        let input = include_str!("../input/test16");

        for line in input.lines() {
            graph.parse_line(line);
        }

        graph.optimize_graph();

        //find the maximum number of different bitmasks
        let num_valves = graph.valves.len();

        let num_bitmasks = 2_u32.pow(num_valves as u32);

        let mut max = 0;
        let mut bitmask_index = HashMap::new();

        for (i, valve) in graph.valves.keys().enumerate() {
            bitmask_index.insert(valve.clone(), i);
        }

        let mut cache = HashMap::new();
        // let mut cache2 = HashMap::new();

        for i in 0..num_bitmasks {
            let pressure_released = graph.dfs("AA".to_string(), 26, i, &bitmask_index, &mut cache)
                + graph.dfs("AA".to_string(), 26, !i, &bitmask_index, &mut cache);

            if pressure_released > max {
                max = pressure_released;
            }
        }
        assert_eq!(max, 1707);
    }
}
