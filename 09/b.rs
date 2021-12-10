use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

fn main() {
    let input : Vec<Vec<_>> = io::stdin().lock().lines().map(|l| l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let mut basins = Vec::new();
    let mut pos_to_basin = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if pos_to_basin.contains_key(&(i,j)) || input[i][j] == 9 {
                continue;
            }
            let basin = basins.len();
            basins.push(0);

            let mut q = VecDeque::from([(i,j)]);

            while !q.is_empty() {
                let (a,b) = q.pop_front().unwrap();
                if pos_to_basin.contains_key(&(a,b)) {
                    continue;
                }
                pos_to_basin.insert((a,b), basin);
                basins[basin] += 1;
                if a > 0 && input[a-1][b] < 9 {
                    q.push_back((a-1,b));
                }
                if a < input.len() - 1 && input[a+1][b] < 9 {
                    q.push_back((a+1,b));
                }
                if b > 0 && input[a][b-1] < 9 {
                    q.push_back((a,b-1));
                }
                if b < input[a].len() - 1 && input[a][b+1] < 9 {
                    q.push_back((a,b+1));
                }
            }
        }
    }
    basins.sort();
    basins.reverse();
    println!("{}", basins[0] * basins[1] * basins[2]);
}
