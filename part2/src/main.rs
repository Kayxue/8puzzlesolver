use std::io::stdin;

const Ndim: usize = 3;
const directions: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let dataCount: u16 = input.trim().parse().unwrap();
    for _ in 0..dataCount {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let data: Vec<u8> = input
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect();
        let mut state: Vec<Vec<u8>> = vec![vec![0; Ndim]; Ndim];
        for i in 0..Ndim {
            for j in 0..Ndim {
                state[i][j] = data[i * Ndim + j]
            }
        }
        let (mut row, mut col) = (0i8, 0i8);
        'checkPosition: {
            for r in 0..Ndim {
                for c in 0..Ndim {
                    if state[r][c] == 0 {
                        row = r as i8;
                        col = c as i8;
                        break 'checkPosition;
                    }
                }
            }
        }
        let mut canMove: Vec<(i8, String)> = Vec::new();
        for (i, dir) in directions.iter().enumerate() {
            let mut copy = state.clone();
            let (newRow, newCol) = (row + dir.0, col + dir.1);
            if newRow >= 0 && newRow < Ndim as i8 && newCol >= 0 && newCol < Ndim as i8 {
                copy[row as usize][col as usize] = copy[newRow as usize][newCol as usize];
                copy[newRow as usize][newCol as usize] = 0;
                let mut result = String::new();
                for r in copy {
                    for c in r {
                        result += &c.to_string();
                    }
                }
                canMove.push((i as i8, result));
            }
        }
        println!("{}", canMove.len());
        for (dir, result) in canMove {
            println!(
                "move 0 to {}",
                match dir {
                    0 => "up",
                    1 => "down",
                    2 => "left",
                    3 => "right",
                    _ => "unknown",
                }
            );
            println!("{result}");
        }
    }
}
