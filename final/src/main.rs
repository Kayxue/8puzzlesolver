use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    io::stdin,
};

const NDIM: usize = 3;
const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const GOAL: [[u8; NDIM]; NDIM] = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
type Matrix = Vec<Vec<u8>>;

#[derive(Clone, Eq, PartialEq)]
struct DataNode<'a> {
    state: &'a Matrix,
    g: i32,
    h: i32,
    f: i32,
    action: u8,
    parent: Option<&'a DataNode<'a>>,
}

impl<'a> DataNode<'a> {
    fn new(state: &'a Matrix, g: i32, h: i32) -> DataNode<'a> {
        DataNode {
            state,
            g,
            h,
            f: g + h,
            action: 0,
            parent: None,
        }
    }
}

impl<'a> Ord for DataNode<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl<'a> PartialOrd for DataNode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solvable(state: &Matrix) -> bool {
    let mut pair_count = 0;
    let flattened: Vec<u8> = state.iter().flatten().cloned().collect();
    for (i, &value) in flattened.iter().enumerate() {
        if value == 0 {
            continue;
        }
        let num = value;
        for j in i + 1..flattened.len() {
            if flattened[j] == 0 {
                continue;
            }
            if num > flattened[j] {
                pair_count += 1;
            }
        }
    }
    pair_count & 1 == 0
}

fn heuristic(state: &Matrix) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..NDIM {
        for j in 0..NDIM {
            let to_find = GOAL[i][j];
            if to_find == 0 {
                continue;
            }
            let (mut row, mut col) = (0i8, 0i8);
            'check_position: {
                for r in 0..NDIM {
                    for c in 0..NDIM {
                        if state[r][c] == to_find {
                            row = r as i8;
                            col = c as i8;
                            break 'check_position;
                        }
                    }
                }
            }
            sum += (row - i as i8).abs() as u32 + (col - j as i8).abs() as u32;
        }
    }
    sum
}

fn find_successor(state: &Matrix) -> (Vec<u8>, Vec<Matrix>) {
    let mut next_direction: Vec<u8> = Vec::new();
    let mut next_state: Vec<Matrix> = Vec::new();
    let (mut row, mut col) = (0i8, 0i8);
    'check_position: {
        for r in 0..NDIM {
            for c in 0..NDIM {
                if state[r][c] == 0 {
                    row = r as i8;
                    col = c as i8;
                    break 'check_position;
                }
            }
        }
    }
    for (i, dir) in DIRECTIONS.iter().enumerate() {
        let mut copy = state.clone();
        let (new_row, new_col) = (row + dir.0, col + dir.1);
        if new_row >= 0 && new_row < NDIM as i8 && new_col >= 0 && new_col < NDIM as i8 {
            copy[row as usize][col as usize] = copy[new_row as usize][new_col as usize];
            copy[new_row as usize][new_col as usize] = 0;
            next_state.push(copy);
            next_direction.push(i as u8);
        }
    }
    (next_direction, next_state)
}

fn goal_reached(state: &Vec<Vec<u8>>) -> bool {
    for i in 0..NDIM {
        for j in 0..NDIM {
            if state[i][j] != GOAL[i][j] {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut final_node: Option<DataNode> = None;
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let data: Vec<u8> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();
    let mut state: Vec<Vec<u8>> = vec![vec![0; NDIM]; NDIM];
    for i in 0..NDIM {
        for j in 0..NDIM {
            state[i][j] = data[i * NDIM + j]
        }
    }
    if !solvable(&state) {
        println!("No solution!!");
        return;
    }
    if goal_reached(&state) {
        println!("It is the goal state.");
        return;
    }
    let mut pqueue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let my_node_i = DataNode::new(&state, 0, heuristic(&state) as i32);
    pqueue.push(my_node_i);
    visited.insert(Box::leak(Box::new(state.clone())));

    while !pqueue.is_empty() {
        let my_node = pqueue.pop().unwrap();
        if goal_reached(&my_node.state) {
            final_node = Some(my_node);
            break;
        }
        let (next_direction, next_state) = find_successor(&my_node.state);
        for (n_direction, n_state) in next_direction.into_iter().zip(next_state.into_iter()) {
            if visited.contains(&n_state) {
                continue;
            }
            let g = my_node.g + 1;
            let h = heuristic(&n_state) as i32;
            let n_node = DataNode {
                state: Box::leak(Box::new(n_state.clone())),
                action: n_direction,
                g,
                h,
                f: g + h,
                parent: Some(Box::leak(Box::new(my_node.clone()))),
            };
            pqueue.push(n_node);
            visited.insert(Box::leak(Box::new(n_state)));
        }
    }
    if let Some(node) = final_node {
        let mut steps: VecDeque<&DataNode> = VecDeque::new();
        let mut now_node = &node;
        while let Some(parent) = now_node.parent {
            steps.push_front(now_node);
            now_node = parent;
        }
        for s in &steps {
            println!(
                "move 0 to {}",
                match s.action {
                    0 => "up",
                    1 => "down",
                    2 => "left",
                    3 => "right",
                    _ => "unknown",
                }
            );
        }
    } else {
        println!("No solution!!")
    }
}
