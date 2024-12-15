use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct GardenPlots {
    plant: char,
    pos: (usize, usize),
}

#[derive(Debug)]
struct Region {
    plant: char,
    plots: Vec<GardenPlots>,
}

#[derive(Debug)]
struct Farm {
    regions: Vec<Region>,
}

fn find_region(
    plot: &GardenPlots,
    region: &mut Region,
    plots: &Vec<Vec<GardenPlots>>,
    max: (u32, u32),
    cache: &mut HashSet<GardenPlots>,
) {
    if cache.contains(&plot) {
        return;
    }

    cache.insert(plot.clone());
    region.plots.push(plot.clone());

    if plot.pos.0 > 0 && plots[plot.pos.1][plot.pos.0 - 1].plant == plot.plant {
        find_region(
            &plots[plot.pos.1][plot.pos.0 - 1],
            region,
            plots,
            max,
            cache,
        );
    }

    if plot.pos.0 < max.0 as usize && plots[plot.pos.1][plot.pos.0 + 1].plant == plot.plant {
        find_region(
            &plots[plot.pos.1][plot.pos.0 + 1],
            region,
            plots,
            max,
            cache,
        );
    }

    if plot.pos.1 > 0 && plots[plot.pos.1 - 1][plot.pos.0].plant == plot.plant {
        find_region(
            &plots[plot.pos.1 - 1][plot.pos.0],
            region,
            plots,
            max,
            cache,
        );
    }

    if plot.pos.1 < max.1 as usize && plots[plot.pos.1 + 1][plot.pos.0].plant == plot.plant {
        find_region(
            &plots[plot.pos.1 + 1][plot.pos.0],
            region,
            plots,
            max,
            cache,
        );
    }
}

impl FromStr for Farm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<Vec<GardenPlots>> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, plant)| GardenPlots { plant, pos: (x, y) })
                    .collect()
            })
            .collect();

        let mut regions: Vec<Region> = vec![];
        let mut cache: HashSet<GardenPlots> = HashSet::new();
        for plot in input.clone().into_iter().flatten() {
            let mut region = Region {
                plant: plot.plant,
                plots: vec![],
            };
            find_region(
                &plot,
                &mut region,
                &input,
                (input[0].len() as u32 - 1, input.len() as u32 - 1),
                &mut cache,
            );

            if region.plots.len() > 0 {
                regions.push(region);
            }
        }

        Ok(Farm { regions })
    }
}

fn part1(input: &str) -> u32 {
    let farm: Farm = input.parse().unwrap();

    let mut total_price = 0;
    for region in farm.regions.iter() {
        let area = region.plots.len();
        let mut region_fences = 0;

        for plot in region.plots.clone() {
            let mut fences = 4;
            for second_plot in region.plots.clone() {
                if plot.pos.0 == second_plot.pos.0 && plot.pos.1 == second_plot.pos.1 + 1 {
                    fences -= 1;
                }
                if plot.pos.0 == second_plot.pos.0 && plot.pos.1 + 1 == second_plot.pos.1 {
                    fences -= 1;
                }
                if plot.pos.0 == second_plot.pos.0 + 1 && plot.pos.1 == second_plot.pos.1 {
                    fences -= 1;
                }
                if plot.pos.0 + 1 == second_plot.pos.0 && plot.pos.1 == second_plot.pos.1 {
                    fences -= 1;
                }
            }
            region_fences += fences;
        }

        total_price += area * region_fences;
    }
    total_price as u32
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum FenceType {
    None,
    HorizontalTop((usize, usize)),
    HorizontalBottom((usize, usize)),
    VerticalLeft((usize, usize)),
    VerticalRight((usize, usize)),
}

fn part2(input: &str) -> u32 {
    let farm: Farm = input.parse().unwrap();

    let mut total_price = 0;

    for region in farm.regions {
        let area = region.plots.len();
        let mut region_fences: Vec<FenceType> = vec![];

        for plot in region.plots.clone() {
            let mut fences = vec![
                FenceType::HorizontalTop(plot.pos),
                FenceType::HorizontalBottom(plot.pos),
                FenceType::VerticalLeft(plot.pos),
                FenceType::VerticalRight(plot.pos),
            ];
            for second_plot in region.plots.clone() {
                if plot.pos.0 == second_plot.pos.0 && plot.pos.1 == second_plot.pos.1 + 1 {
                    fences[0] = FenceType::None;
                }
                if plot.pos.0 == second_plot.pos.0 && plot.pos.1 + 1 == second_plot.pos.1 {
                    fences[1] = FenceType::None;
                }
                if plot.pos.0 == second_plot.pos.0 + 1 && plot.pos.1 == second_plot.pos.1 {
                    fences[2] = FenceType::None;
                }
                if plot.pos.0 + 1 == second_plot.pos.0 && plot.pos.1 == second_plot.pos.1 {
                    fences[3] = FenceType::None;
                }
            }
            region_fences.extend(fences.into_iter().filter(|fence| *fence != FenceType::None))
        }

        let mut fences_checked = HashSet::new();
        let mut num_sides = 0;

        while let Some(fence) = region_fences.pop() {
            if fences_checked.contains(&fence) {
                continue;
            }

            let mut start_pos = match fence {
                FenceType::HorizontalTop(pos) => pos,
                FenceType::HorizontalBottom(pos) => pos,
                FenceType::VerticalLeft(pos) => pos,
                FenceType::VerticalRight(pos) => pos,
                _ => panic!("Invalid fence type"),
            };

            let mut end_pos = match fence {
                FenceType::HorizontalTop(pos) => pos,
                FenceType::HorizontalBottom(pos) => pos,
                FenceType::VerticalLeft(pos) => pos,
                FenceType::VerticalRight(pos) => pos,
                _ => panic!("Invalid fence type"),
            };

            let mut found_connected_fence = true;

            while found_connected_fence {
                found_connected_fence = false;
                for second_fence in region_fences.clone() {
                    if second_fence == fence || fences_checked.contains(&second_fence) {
                        continue;
                    }

                    match (fence.clone(), second_fence.clone()) {
                        (FenceType::HorizontalTop(_), FenceType::HorizontalTop(pos2)) => {
                            if start_pos.0 == pos2.0 + 1 && start_pos.1 == pos2.1 {
                                fences_checked.insert(second_fence);
                                start_pos = pos2;
                                found_connected_fence = true;
                                break;
                            } else if end_pos.0 + 1 == pos2.0 && end_pos.1 == pos2.1 {
                                fences_checked.insert(second_fence);
                                end_pos = pos2;
                                found_connected_fence = true;
                                break;
                            }
                        }
                        (FenceType::HorizontalBottom(_), FenceType::HorizontalBottom(pos2)) => {
                            if start_pos.0 == pos2.0 + 1 && start_pos.1 == pos2.1 {
                                fences_checked.insert(second_fence);
                                start_pos = pos2;
                                found_connected_fence = true;
                                break;
                            } else if end_pos.0 + 1 == pos2.0 && end_pos.1 == pos2.1 {
                                fences_checked.insert(second_fence);
                                end_pos = pos2;
                                found_connected_fence = true;
                                break;
                            }
                        }
                        (FenceType::VerticalLeft(_), FenceType::VerticalLeft(pos2)) => {
                            if start_pos.0 == pos2.0 && start_pos.1 == pos2.1 + 1 {
                                fences_checked.insert(second_fence);
                                start_pos = pos2;
                                found_connected_fence = true;
                                break;
                            } else if end_pos.0 == pos2.0 && end_pos.1 + 1 == pos2.1 {
                                fences_checked.insert(second_fence);
                                end_pos = pos2;
                                found_connected_fence = true;
                                break;
                            }
                        }
                        (FenceType::VerticalRight(_), FenceType::VerticalRight(pos2)) => {
                            if start_pos.0 == pos2.0 && start_pos.1 == pos2.1 + 1 {
                                fences_checked.insert(second_fence);
                                start_pos = pos2;
                                found_connected_fence = true;
                                break;
                            } else if end_pos.0 == pos2.0 && end_pos.1 + 1 == pos2.1 {
                                fences_checked.insert(second_fence);
                                end_pos = pos2;
                                found_connected_fence = true;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            num_sides += 1;
        }

        total_price += area * num_sides;
    }

    total_price as u32
}

fn main() {
    println!("AoC 2024 - Day 1");
    let input = include_str!("input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("test1");
        assert_eq!(part1(input), 140);
    }

    #[test]
    fn part1_test2() {
        let input = include_str!("test2");
        assert_eq!(part1(input), 1930);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("test1");
        assert_eq!(part2(input), 80);
    }

    #[test]
    fn part2_test2() {
        let input = include_str!("test2");
        assert_eq!(part2(input), 1206);
    }

    #[test]
    fn part2_test3() {
        let input = include_str!("test3");
        assert_eq!(part2(input), 236);
    }

    #[test]
    fn part2_test4() {
        let input = include_str!("test4");
        assert_eq!(part2(input), 368);
    }
}
