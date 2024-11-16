use core::panic;
use std::option::Option::{None, Some};
use std::{collections::VecDeque, u8, usize};

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Node {
    height: u8,
    position: Position,
}

enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Directions {
    fn delta(&self) -> (isize, isize) {
        match self {
            Directions::UP => (-1, 0),
            Directions::DOWN => (1, 0),
            Directions::LEFT => (0, -1),
            Directions::RIGHT => (0, 1),
        }
    }

    fn directions() -> [Directions; 4] {
        [
            Directions::UP,
            Directions::DOWN,
            Directions::LEFT,
            Directions::RIGHT,
        ]
    }
}

fn to_height(character: char) -> u8 {
    if character.is_ascii_lowercase() {
        return character as u8 % 32;
    }
    match character {
        'S' => 0,
        'E' => 27,
        _ => panic!("Nothing else in input space"),
    }
}

fn find_start(nodes: &[Node]) -> Position {
    let start = nodes
        .iter()
        .filter(|node| node.height == 0)
        .map(|node| node.position)
        .collect::<Vec<Position>>();
    *start.first().expect("No start found")
}

fn find_end(nodes: &[Node]) -> Position {
    let end = nodes
        .iter()
        .filter(|node| node.height == 27)
        .map(|node| node.position)
        .collect::<Vec<Position>>();
    *end.first().expect("No end found")
}

fn get_index(pos: Position, cols: usize) -> usize {
    pos.0 * (cols + 1) + pos.1
}

fn filter_bounds<'a>(
    cur_node: &'a Node,
    rows: &'a usize,
    cols: &'a usize,
) -> Box<dyn FnMut(&(isize, isize)) -> bool + 'a> {
    Box::new(|delta| {
        let x = cur_node
            .position
            .0
            .checked_add_signed(delta.0)
            .map_or(false, |val| val <= *rows);
        let y = cur_node
            .position
            .1
            .checked_add_signed(delta.1)
            .map_or(false, |val| val <= *cols);
        x && y
    })
}

fn bfs(
    graph: &[Node],
    start: Position,
    parents: &mut Vec<Option<Position>>,
    distances: &mut Vec<usize>,
) {
    let (rows, cols) = graph.last().expect("Graph is empty.").position;
    distances[get_index(start, cols)] = 0;
    let mut visited: VecDeque<Node> = VecDeque::new();
    visited.push_back(graph[get_index(start, cols)].clone());
    while !visited.is_empty() {
        let cur_node = visited.pop_front().expect("No node to pop");
        let dist = distances.clone();
        Directions::directions()
            .iter()
            .map(|direction| direction.delta())
            .filter(filter_bounds(&cur_node, &rows, &cols))
            .map(|delta| {
                (
                    cur_node.position.0.saturating_add_signed(delta.0),
                    cur_node.position.1.saturating_add_signed(delta.1),
                )
            })
            .filter(|position| dist[get_index(*position, cols)] == usize::MAX)
            .map(|position| graph[position.0 * (cols + 1) + position.1].clone())
            .filter(|node| {
                let overflow = node.height.overflowing_sub(cur_node.height);
                overflow.0 <= 1 || overflow.1
            })
            .for_each(|node| {
                parents[get_index(node.position, cols)] = Some(cur_node.position);
                distances[get_index(node.position, cols)] =
                    distances[get_index(cur_node.position, cols)] + 1;
                visited.push_back(node.clone());
            });
    }
}

fn bfs_to_a(graph: &[Node], start: Position, distances: &mut Vec<usize>) -> usize {
    let (rows, cols) = graph.last().expect("Graph is empty.").position;
    distances[get_index(start, cols)] = 0;
    let mut visited: VecDeque<Node> = VecDeque::new();
    visited.push_back(graph[get_index(start, cols)].clone());
    while !visited.is_empty() {
        let cur_node = visited.pop_front().expect("No node to pop");
        if cur_node.height == 0 || cur_node.height == 1 {
            return distances[get_index(cur_node.position, cols)];
        }
        let dist = distances.clone();
        Directions::directions()
            .iter()
            .map(|direction| direction.delta())
            .filter(filter_bounds(&cur_node, &rows, &cols))
            .map(|delta| {
                (
                    cur_node.position.0.saturating_add_signed(delta.0),
                    cur_node.position.1.saturating_add_signed(delta.1),
                )
            })
            .filter(|position| dist[get_index(*position, cols)] == usize::MAX)
            .map(|position| graph[position.0 * (cols + 1) + position.1].clone())
            .filter(|node| {
                let overflow = cur_node.height.overflowing_sub(node.height);
                overflow.0 <= 1 || overflow.1
            })
            .for_each(|node| {
                visited.push_back(node.clone());
                distances[get_index(node.position, cols)] =
                    distances[get_index(cur_node.position, cols)] + 1;
            });
    }
    usize::MAX
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Node> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| Node {
                    height: to_height(char),
                    position: (row, col),
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect()
}

#[aoc(day12, part1)]
fn shortest_path(graph: &[Node]) -> usize {
    let (rows, cols) = graph.last().expect("Graph is empty.").position;
    let mut distances = vec![usize::MAX; (rows + 1) * (cols + 1)];
    let mut parents = vec![None; (rows + 1) * (cols + 1)];
    bfs(&graph, find_start(&graph), &mut parents, &mut distances);
    let mut path: VecDeque<usize> = VecDeque::new();
    let end_pos = find_end(&graph);
    path.push_back(get_index(end_pos, cols));
    let mut cur_node = get_index(end_pos, cols);
    while !parents[cur_node].is_none() {
        path.push_back(get_index(parents[cur_node].expect("No parent node"), cols));
        cur_node = get_index(parents[cur_node].expect("No parent node"), cols);
    }
    path.len() - 1
}

#[aoc(day12, part2)]
fn shortest_path_to_a(graph: &[Node]) -> usize {
    let (rows, cols) = graph.last().expect("Graph is empty.").position;
    let mut distances = vec![usize::MAX; (rows + 1) * (cols + 1)];
    bfs_to_a(&graph, find_end(&graph), &mut distances)
}

#[cfg(test)]
mod test {
    use crate::day12::{find_end, find_start, shortest_path, shortest_path_to_a, Position};

    use super::{parse_input, to_height, Node};
    const SAMPLE_MAP: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    const MIN: &str = "Sab
cdE";

    #[test]
    fn to_height_lowercase() {
        assert_eq!(1, to_height('a'));
        assert_eq!(2, to_height('b'));
        assert_eq!(25, to_height('y'));
        assert_eq!(26, to_height('z'));
    }

    #[test]
    fn to_height_uppercase() {
        assert_eq!(0, to_height('S'));
        assert_eq!(27, to_height('E'));
    }

    #[test]
    fn min_parse_input() {
        let expected: Vec<Node> = vec![
            Node {
                height: 0,
                position: (0, 0),
            },
            Node {
                height: 1,
                position: (0, 1),
            },
            Node {
                height: 2,
                position: (0, 2),
            },
            Node {
                height: 3,
                position: (1, 0),
            },
            Node {
                height: 4,
                position: (1, 1),
            },
            Node {
                height: 27,
                position: (1, 2),
            },
        ];
        assert_eq!(expected, parse_input(MIN))
    }

    #[test]
    fn sample_find_start() {
        let expected: Position = (0, 0);
        let nodes = parse_input(SAMPLE_MAP);
        assert_eq!(expected, find_start(&nodes))
    }

    #[test]
    fn sample_find_end() {
        let expected: Position = (2, 5);
        let nodes = parse_input(SAMPLE_MAP);
        assert_eq!(expected, find_end(&nodes))
    }

    #[test]
    fn sample_shortest_path_from_s() {
        let expected = 31;
        let nodes = parse_input(SAMPLE_MAP);
        let actual = shortest_path(&nodes);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_shortest_path_to_a() {
        let expected = 29;
        let nodes = parse_input(SAMPLE_MAP);
        let actual = shortest_path_to_a(&nodes);
        assert_eq!(expected, actual);
    }
}
