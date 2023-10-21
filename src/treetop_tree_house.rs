use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Tree {
    visible: bool,
    height: u8,
    scenic_scores: HashMap<u8, usize>,
}

impl Tree {
    fn new(height: u8) -> Self {
        Tree {
            visible: false,
            height,
            scenic_scores: HashMap::new(),
        }
    }

    fn update_scenic_score(&mut self, cardinality: u8, vectors: Vec<&Tree>)
    {
        self.scenic_scores.insert(cardinality, 0);
        for tree in vectors {
            *self.scenic_scores.entry(cardinality).or_insert(0) += 1;
            if tree.height >= self.height {
                return;
            }
        }
        self.visible = true;
    }

    fn get_scenic_score(&self) -> usize {
        let mut product = 1;
        self.scenic_scores.values().for_each(|&score| product *= score);
        product
    }
}

#[derive(Debug)]
struct ForestMap {
    trees: Vec<Tree>,
    num_cols: usize,
    num_rows: usize,
}

impl FromStr for ForestMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = ForestMap::new();
        for line in s.lines() {
            for byte in line.bytes() {
                map.push(Tree::new(byte - 48));
            }
            map.push_row();
        }
        Ok(map)
    }
}

impl ForestMap {
    fn new() -> Self {
        ForestMap {
            trees: Vec::new(),
            num_cols: 0,
            num_rows: 0,
        }
    }

    fn push_row(&mut self) {
        if self.num_cols == 0 {
            self.num_cols = self.trees.len()
        }
        self.num_rows += 1;
    }

    fn push(&mut self, tree: Tree) {
        if self.num_cols == 0 { 0 } else { self.trees.len() / self.num_cols };
        self.trees.push(tree);
    }

    fn get_up_indices(&self, location: (usize, usize)) -> Vec<usize> {
        (0..(location.0 + location.1 * self.num_cols)).rev().filter(move |x| {
            *x % self.num_cols == location.0
        }).collect()
    }

    fn get_down_indices(&self, location: (usize, usize)) -> Vec<usize> {
        ((location.0 + location.1 * self.num_cols + 1)..self.trees.len()).filter(move |x| {
            *x % self.num_cols == location.0
        }).collect()
    }

    fn get_right_indices(&self, location: (usize, usize)) -> Vec<usize> {
        ((location.0 + location.1 * self.num_cols + 1)..self.num_cols * (location.1 + 1)).collect()
    }

    fn get_left_indices(&self, location: (usize, usize)) -> Vec<usize> {
        (self.num_cols * (location.1)..(location.0 + location.1 * self.num_cols)).rev().collect()
    }

    fn get_tree(&self, location: (usize, usize)) -> &Tree {
        self.trees.get(location.0 + location.1 * self.num_cols).unwrap()
    }

    fn set_scenic_scores(&mut self)
    {
        for col in 0..self.num_cols {
            for row in 0..self.num_rows {
                let immut_trees: Vec<Tree> = self.trees.clone();
                let up_trees: Vec<&Tree> = immut_trees.iter()
                    .enumerate()
                    .filter(|(index, _)| self.get_up_indices((col, row)).contains(index))
                    .map(|(_, tree)| tree)
                    .collect();
                let down_trees = immut_trees.iter()
                    .enumerate()
                    .filter(|(index, _)| self.get_down_indices((col, row)).contains(index))
                    .map(|(_, tree)| tree)
                    .collect();
                let right_trees = immut_trees.iter()
                    .enumerate()
                    .filter(|(index, _)| self.get_right_indices((col, row)).contains(index))
                    .map(|(_, tree)| tree)
                    .collect();
                let left_trees: Vec<&Tree> = immut_trees.iter()
                    .enumerate()
                    .filter(|(index, _)| self.get_left_indices((col, row)).contains(index))
                    .map(|(_, tree)| tree)
                    .collect();

                self.trees.get_mut(col + row * self.num_cols)
                    .unwrap()
                    .update_scenic_score(
                        0,
                        up_trees.into_iter().rev().collect(),
                    );

                self.trees.get_mut(col + row * self.num_cols)
                    .unwrap()
                    .update_scenic_score(
                        1,
                        down_trees,
                    );

                self.trees.get_mut(col + row * self.num_cols)
                    .unwrap()
                    .update_scenic_score(
                        2,
                        right_trees,
                    );

                self.trees.get_mut(col + row * self.num_cols)
                    .unwrap()
                    .update_scenic_score(
                        3,
                        left_trees.into_iter().rev().collect(),
                    );
            }
        }
    }

    fn print_visible(&self) {
        let vec_idx = |col: usize, row: usize| col + row * self.num_cols;

        for col in 0..self.num_cols {
            for row in 0..self.num_rows {
                let visible = if self.trees.get(vec_idx(col, row)).unwrap().visible { "X" } else {
                    " "
                };
                print!("{}", visible);
            }
            println!();
        }
    }
}

#[aoc_generator(day8)]
fn create_visibility_map(input: &str) -> ForestMap {
    let mut map = ForestMap::from_str(input).expect("Parse error");
    map.set_scenic_scores();
    map
}

#[aoc(day8, part1)]
fn count_visible(input: &ForestMap) -> usize {
    input.trees.iter().map(|tree| {
        if tree.visible { 1 } else { 0 }
    }).sum()
}

#[aoc(day8, part2)]
fn highest_scenic_score(input: &ForestMap) -> usize {
    let mut trees = input.trees.clone();

    trees.sort_by(|a, b| a.get_scenic_score().partial_cmp(&b.get_scenic_score()).unwrap());

    trees.last().unwrap().get_scenic_score()
}

fn _highest_scenic_score(p0: &ForestMap) -> usize {
    let mut trees = p0.trees.clone();

    trees.sort_by(|a, b| a.get_scenic_score().partial_cmp(&b.get_scenic_score()).unwrap());

    trees.last().unwrap().get_scenic_score()
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::treetop_tree_house::{_highest_scenic_score, count_visible, ForestMap};

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn sample_test_part1() {
        let mut map = ForestMap::from_str(EXAMPLE).expect("Parse error");

        map.set_scenic_scores();
        map.print_visible();

        dbg!(&map);
        assert_eq!(21, count_visible(&map));
    }

    #[test]
    fn sample_test_part2() {
        let mut map = ForestMap::from_str(EXAMPLE).expect("Parse error");

        map.set_scenic_scores();
        dbg!(&map);
        assert_eq!(8, _highest_scenic_score(&map))
    }
}
