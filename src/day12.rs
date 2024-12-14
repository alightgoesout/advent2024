use std::cell::OnceCell;
use std::collections::{BTreeSet, HashSet};

use crate::{Result, Solution};

mod input;

#[derive(Default)]
pub struct Day12(OnceCell<PlotMap>);

impl Day12 {
    fn plot_map(&self) -> &PlotMap {
        self.0.get_or_init(|| PlotMap::parse::<140>(input::INPUT))
    }
}

impl Solution for Day12 {
    fn part_one(&self) -> Result<String> {
        let fencing_price = self.plot_map().fencing_price();
        Ok(format!("Fencing price: {fencing_price}"))
    }

    fn part_two(&self) -> Result<String> {
        let fencing_price = self.plot_map().fencing_price_using_sides();
        Ok(format!("Fencing price using sides: {fencing_price}"))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Plot {
    column: i32,
    row: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Region {
    plant: char,
    plots: HashSet<Plot>,
}

impl Region {
    fn fencing_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn fencing_price_using_sides(&self) -> usize {
        self.area() * self.count_sides()
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.plots
            .iter()
            .map(|plot| 4 - self.count_adjacent_plots(plot.column, plot.row))
            .sum()
    }

    fn count_sides(&self) -> usize {
        let mut sides = 0;
        for &Plot { column, row } in &self.plots {
            if (row == 0 || !self.contains(column, row - 1))
                && (!self.contains(column - 1, row) || self.contains(column - 1, row - 1))
            {
                sides += 1;
            }
            if !self.contains(column + 1, row)
                && (!self.contains(column, row - 1) || self.contains(column + 1, row - 1))
            {
                sides += 1;
            }
            if (!self.contains(column, row + 1))
                && (!self.contains(column + 1, row) || self.contains(column + 1, row + 1))
            {
                sides += 1;
            }
            if (column == 0 || !self.contains(column - 1, row))
                && (!self.contains(column, row + 1) || self.contains(column - 1, row + 1))
            {
                sides += 1;
            }
        }
        sides
    }

    fn contains(&self, column: i32, row: i32) -> bool {
        self.plots.contains(&Plot { column, row })
    }

    fn count_adjacent_plots(&self, column: i32, row: i32) -> usize {
        let mut adjacent_plots = 0;

        if self.contains(column - 1, row) {
            adjacent_plots += 1;
        }
        if self.contains(column + 1, row) {
            adjacent_plots += 1;
        }
        if self.contains(column, row - 1) {
            adjacent_plots += 1;
        }
        if self.contains(column, row + 1) {
            adjacent_plots += 1;
        }

        adjacent_plots
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PlotMap(Vec<Region>);

impl PlotMap {
    fn parse<const N: usize>(s: &str) -> Self {
        let plants = parse_plants::<N>(s);
        let mut regions = Vec::new();
        let mut visited_plots = HashSet::new();

        for row in 0..(N as i32) {
            for column in 0..(N as i32) {
                let plot = Plot { column, row };
                if visited_plots.contains(&plot) {
                    continue;
                }
                let plant = plants[plot.row as usize][plot.column as usize];
                let plots = find_region_plots(&plants, plant, plot);
                visited_plots.extend(plots.clone());
                regions.push(Region { plant, plots });
            }
        }

        PlotMap(regions)
    }

    fn fencing_price(&self) -> usize {
        self.0.iter().map(Region::fencing_price).sum()
    }

    fn fencing_price_using_sides(&self) -> usize {
        self.0.iter().map(Region::fencing_price_using_sides).sum()
    }
}

fn parse_plants<const N: usize>(s: &str) -> [[char; N]; N] {
    let mut plots = [['.'; N]; N];
    for (i, plant) in s.chars().enumerate() {
        if i % (N + 1) == N {
            continue;
        }
        plots[i / (N + 1)][i % (N + 1)] = plant;
    }
    plots
}

fn find_region_plots<const N: usize>(
    plants: &[[char; N]; N],
    plant: char,
    plot: Plot,
) -> HashSet<Plot> {
    let mut plots = HashSet::new();
    let mut to_visit = BTreeSet::from([plot]);

    while let Some(plot) = to_visit.pop_first() {
        if plot.column < N as i32 - 1 {
            let right_plot = Plot {
                column: plot.column + 1,
                row: plot.row,
            };
            if plants[right_plot.row as usize][right_plot.column as usize] == plant
                && !plots.contains(&right_plot)
            {
                to_visit.insert(right_plot);
            }
        }
        if plot.column > 0 {
            let left_plot = Plot {
                column: plot.column - 1,
                row: plot.row,
            };
            if plants[left_plot.row as usize][left_plot.column as usize] == plant
                && !plots.contains(&left_plot)
            {
                to_visit.insert(left_plot);
            }
        }
        if plot.row < (N as i32) - 1 {
            let bottom_plot = Plot {
                column: plot.column,
                row: plot.row + 1,
            };
            if plants[bottom_plot.row as usize][bottom_plot.column as usize] == plant
                && !plots.contains(&bottom_plot)
            {
                to_visit.insert(bottom_plot);
            }
        }
        if plot.row > 0 {
            let top_plot = Plot {
                column: plot.column,
                row: plot.row - 1,
            };
            if plants[top_plot.row as usize][top_plot.column as usize] == plant
                && !plots.contains(&top_plot)
            {
                to_visit.insert(top_plot);
            }
        }
        plots.insert(plot);
    }

    plots
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_EXAMPLE: &str = "\
AAAA
BBCD
BBCC
EEEC
";

    const LARGE_EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn one_plot_region_perimeter() {
        let region = Region {
            plant: 'A',
            plots: HashSet::from([Plot { column: 0, row: 0 }]),
        };

        assert_eq!(region.fencing_price(), 4);
    }

    #[test]
    fn two_plots_region_perimeter() {
        let region = Region {
            plant: 'A',
            plots: HashSet::from([Plot { column: 0, row: 0 }, Plot { column: 1, row: 0 }]),
        };

        assert_eq!(region.fencing_price(), 12);
    }

    #[test]
    fn fencing_price_small_example() {
        let plot_map = PlotMap::parse::<4>(SMALL_EXAMPLE);

        assert_eq!(plot_map.fencing_price(), 140);
    }

    #[test]
    fn fencing_price_large_example() {
        let plot_map = PlotMap::parse::<10>(LARGE_EXAMPLE);

        dbg!(&plot_map.0.len());

        assert_eq!(plot_map.fencing_price(), 1930);
    }

    #[test]
    fn region_count_sides_one_plot() {
        let region = Region {
            plant: 'A',
            plots: HashSet::from([Plot { column: 0, row: 0 }]),
        };

        assert_eq!(region.count_sides(), 4);
    }

    #[test]
    fn region_count_sides_two_horizontal_plots() {
        let region = Region {
            plant: 'A',
            plots: HashSet::from([Plot { column: 0, row: 0 }, Plot { column: 1, row: 0 }]),
        };

        assert_eq!(region.count_sides(), 4);
    }

    #[test]
    fn region_count_sides_two_vertical_plots() {
        let region = Region {
            plant: 'A',
            plots: HashSet::from([Plot { column: 0, row: 0 }, Plot { column: 0, row: 1 }]),
        };

        assert_eq!(region.count_sides(), 4);
    }

    #[test]
    fn region_count_sides_e_example() {
        let plot_map = PlotMap::parse::<5>(
            "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );
        let region = plot_map
            .0
            .iter()
            .find(|region| region.plant == 'E')
            .unwrap();

        assert_eq!(region.count_sides(), 12);
    }

    #[test]
    fn region_count_sides_a_example() {
        let plot_map = PlotMap::parse::<6>(
            "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );
        let region = plot_map
            .0
            .iter()
            .find(|region| region.plant == 'A')
            .unwrap();

        assert_eq!(region.count_sides(), 12);
    }

    #[test]
    fn fencing_price_using_sides_e_example() {
        let plot_map = PlotMap::parse::<5>(
            "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );

        assert_eq!(plot_map.fencing_price_using_sides(), 236);
    }

    #[test]
    fn fencing_price_using_sides_last_example() {
        let plot_map = PlotMap::parse::<6>(
            "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );

        assert_eq!(plot_map.fencing_price_using_sides(), 368);
    }
}
