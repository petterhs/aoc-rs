use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(Clone, Debug, PartialEq)]
struct Valve {
    name: String,
    flow_rate: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct Tunnel {
    from: String,
    to: String,
    time: i32,
}

impl Tunnel {
    fn new(from: &str, to: &str, time: i32) -> Tunnel {
        Tunnel {
            from: from.to_string(),
            to: to.to_string(),
            time,
        }
    }
}

#[derive(Debug)]
struct Graph {
    valves: HashMap<String, Valve>,
    tunnels: HashMap<String, Vec<Tunnel>>,
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
                .push(Tunnel::new(from, &tunnel, time));
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
        //Remove valves with 0 flow_rate
        //and connect tunnels to each other
        //with the time it takes to travel the tunnel

        let mut valves_to_remove = vec![];

        //Remove valves with 0 flow_rate
        self.valves.iter().for_each(|(valve, valve_info)| {
            if valve_info.flow_rate == 0 {
                valves_to_remove.push(valve.to_string());
            }
        });

        valves_to_remove.iter().for_each(|valve| {
            let valvename = valve.to_string();
            let outputs = self.tunnels[valve].clone();

            let valve = self.valves.get(&valvename).unwrap();

            self.tunnels
                .iter_mut()
                .filter(|(_, v)| v.iter().any(|tunnel| tunnel.to == valve.name))
                .for_each(|(k, v)| {
                    let time = v
                        .iter()
                        .find(|tunnel| tunnel.to == valve.name)
                        .unwrap()
                        .time;

                    //delete tunnel to removed valve
                    v.retain(|tunnel| tunnel.to != valve.name);

                    //add tunnels to all outputs
                    for output in &outputs {
                        if output.to == *k {
                            continue;
                        }
                        v.iter_mut().for_each(|tunnel| {
                            if tunnel.to == output.to {
                                if tunnel.time > output.time + 1 {
                                    tunnel.time = output.time + 1;
                                }
                            }
                        });

                        if v.iter().any(|tunnel| tunnel.to == output.to) {
                            continue;
                        }

                        v.push(Tunnel::new(k, &output.to, output.time + time));
                    }
                });

            if valvename != "AA" {
                self.valves.remove(valvename.as_str());
                self.tunnels.remove(valvename.as_str());
            }
        });
    }

    fn df_max_flow_rate(&mut self, from: &str) -> i32 {
        let time_left = 30;
        let pressure_released = 0;

        let opened_valves: Vec<&str> = Vec::new();
        let mut stack = Vec::new();
        stack.push((
            from.to_string(),
            opened_valves,
            time_left,
            pressure_released,
        ));

        let mut max = 0;

        let mut cache = HashSet::new();

        while let Some((valve, opened_valves, time_left, pressure_released)) = stack.pop() {
            if time_left <= 0 {
                max = max.max(pressure_released);
                continue;
            }

            if opened_valves.len() == self.valves.len() {
                max = max.max(pressure_released);
                continue;
            }

            if cache.contains(&(valve.clone(), opened_valves.clone(), time_left)) {
                max = max.max(pressure_released);
                continue;
            }

            // println!(
            //     "Valve: {} {:?}{} {}",
            //     valve, opened_valves, time_left, pressure_released
            // );

            // println!("{}", time_left);
            let valve = self.valves.get(&valve).unwrap();
            for tunnel in self.tunnels.get(&valve.name).unwrap() {
                //Not opening current valve
                // println!("Pushing valve: {}", tunnel.to);
                stack.push((
                    tunnel.to.to_string(),
                    opened_valves.clone(),
                    time_left - tunnel.time,
                    pressure_released,
                ));

                if opened_valves.contains(&valve.name.as_str()) {
                    continue;
                }
                // //Opening current valve

                let mut opened_valves = opened_valves.clone();
                opened_valves.push(&valve.name);

                let pressure_released = pressure_released + valve.flow_rate * (time_left - 1);
                // println!(
                //     "Opening valve: {} and Pushing valve: {}",
                //     valve.name, tunnel.to
                // );
                stack.push((
                    tunnel.to.to_string(),
                    opened_valves,
                    time_left - tunnel.time - 1,
                    pressure_released,
                ));
            }

            cache.insert((valve.name.clone(), opened_valves, time_left));
        }

        // println!("{:?}", possibilites);
        max
    }
}

fn part1() -> i32 {
    let mut graph = Graph::new();
    let input = include_str!("../input/16");

    for line in input.lines() {
        graph.parse_line(line);
    }

    graph.optimize_graph();
    graph.df_max_flow_rate("AA")
}

fn part2() -> i32 {
    0
}

pub fn run() {
    println!("Day 16");
    println!("Part1: {}", part1());
    part2();
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

        println!("{}", graph);

        graph.optimize_graph();

        assert_eq!(graph.valves.len(), 7);
        assert_eq!(graph.tunnels.len(), 7);

        println!("{}", graph);

        let mut tunnel_aa = graph.tunnels.get("AA").unwrap().clone();
        tunnel_aa.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnel_aa,
            vec![
                Tunnel::new("AA", "BB", 1),
                Tunnel::new("AA", "DD", 1),
                Tunnel::new("AA", "JJ", 2),
            ]
        );

        let mut tunnel_bb = graph.tunnels.get("BB").unwrap().clone();
        tunnel_bb.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnel_bb,
            vec![
                Tunnel::new("BB", "CC", 1),
                Tunnel::new("BB", "DD", 2),
                Tunnel::new("BB", "JJ", 3),
            ]
        );

        let mut tunnel_cc = graph.tunnels.get("CC").unwrap().clone();
        tunnel_cc.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnel_cc,
            vec![Tunnel::new("CC", "BB", 1), Tunnel::new("CC", "DD", 1),]
        );

        let mut tunnel_dd = graph.tunnels.get("DD").unwrap().clone();
        tunnel_dd.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnel_dd,
            vec![
                Tunnel::new("DD", "BB", 2),
                Tunnel::new("DD", "CC", 1),
                Tunnel::new("DD", "EE", 1),
                Tunnel::new("DD", "JJ", 3),
            ]
        );

        let mut tunnel_ee = graph.tunnels.get("EE").unwrap().clone();
        tunnel_ee.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnel_ee,
            vec![Tunnel::new("EE", "DD", 1), Tunnel::new("EE", "HH", 3),]
        );

        let mut tunnel_hh = graph.tunnels.get("HH").unwrap().clone();
        tunnel_hh.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(tunnel_hh, vec![Tunnel::new("HH", "EE", 3),]);

        let mut tunnel_jj = graph.tunnels.get("JJ").unwrap().clone();
        tunnel_jj.sort_by(|a, b| a.to.cmp(&b.to));

        assert_eq!(
            tunnel_jj,
            vec![Tunnel::new("JJ", "BB", 3), Tunnel::new("JJ", "DD", 3),]
        );
    }

    #[test]
    fn test_part1() {
        let mut graph = Graph::new();
        let input = include_str!("../input/test16");

        for line in input.lines() {
            graph.parse_line(line);
        }

        graph.optimize_graph();

        assert_eq!(graph.df_max_flow_rate("AA"), 1651);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
