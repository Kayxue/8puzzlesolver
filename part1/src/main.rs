use std::io::stdin;

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
        let mut pairCount = 0;
        for (i, &value) in data.iter().enumerate() {
            if value == 0 {
                continue;
            }
            let num = value;
            for j in i + 1..data.len() {
                if data[j] == 0 {
                    continue;
                }
                if num > data[j] {
                    pairCount += 1;
                }
            }
        }
        if pairCount & 1 == 1 {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}
