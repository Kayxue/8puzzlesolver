use std::{collections::VecDeque, io::stdin};

const Ndim: usize = 3;

struct priorityQueue {
    queue: VecDeque<dataNode>,
}

impl priorityQueue {
    fn new() -> Self {
        priorityQueue {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, node: dataNode) {
        if self.queue.is_empty() {
            self.queue.push_back(node);
        } else {
            if self.queue.front().unwrap().f > node.f {
                self.queue.push_front(node);
            } else if self.queue.back().unwrap().f < node.f {
                self.queue.push_back(node);
            } else {
                let mut inserted = false;
                for i in 0..self.queue.len() {
                    if self.queue[i].f > node.f {
                        self.queue.insert(i, node.clone());
                        inserted = true;
                        break;
                    }
                }
                if !inserted {
                    self.queue.push_back(node);
                }
            }
        }
    }

    fn pop(&mut self) -> Option<dataNode> {
        self.queue.pop_front()
    }
}

#[derive(Clone)]
struct dataNode {
    state: Vec<Vec<u8>>,
    g: i32,
    h: i32,
    f: i32,
}

fn main() {
    let mut pqueue = priorityQueue::new();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let dataCount: u16 = input.trim().parse().unwrap();
    for _ in 0..dataCount {
        let mut inputOperation = String::new();
        stdin().read_line(&mut inputOperation).unwrap();
        if inputOperation.trim() == "enqueue" {
            let mut inputData = String::new();
            stdin().read_line(&mut inputData).unwrap();
            let dataVec: Vec<&str> = inputData.split_whitespace().collect();
            let data: Vec<u8> = dataVec[0]
                .trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect();
            let (g, h): (i32, i32) = (dataVec[1].parse().unwrap(), dataVec[2].parse().unwrap());
            let mut state: Vec<Vec<u8>> = vec![vec![0; Ndim]; Ndim];
            for i in 0..Ndim {
                for j in 0..Ndim {
                    state[i][j] = data[i * Ndim + j]
                }
            }
            let mut node: dataNode = dataNode {
                state,
                g,
                h,
                f: g + h,
            };
            pqueue.push(node);
            println!("Insert OK!");
        } else if inputOperation.trim() == "dequeue" {
            if let Some(node) = pqueue.pop() {
                let str = node
                    .state
                    .into_iter()
                    .flat_map(|row| row.into_iter())
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                println!("Got {}", str);
            } else {
                println!("Queue is empty!!");
            }
        }
    }
}
