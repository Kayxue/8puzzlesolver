use std::io::stdin;

const Ndim: usize = 3;
const goal: [[u8; Ndim]; Ndim] = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];

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
        let mut sum = 0;
        for i in 0..Ndim {
            for j in 0..Ndim {
                let toFind = goal[i][j];
                if toFind == 0 {
                    continue;
                }
                let (mut row, mut col) = (0i8, 0i8);
                'checkPosition: {
                    for r in 0..Ndim {
                        for c in 0..Ndim {
                            if state[r][c] == toFind {
                                row = r as i8;
                                col = c as i8;
                                break 'checkPosition;
                            }
                        }
                    }
                }
                sum += (row - i as i8).abs() + (col - j as i8).abs();
            }
        }
        println!("{sum}")
    }
}
