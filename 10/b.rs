use std::io;
use std::io::BufRead;

fn pair(c : char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None
    }
}

fn main() {
    let mut scores : Vec<u64> = Vec::new();
    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let mut stack : Vec<char> = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            let opening = pair(c);
            match opening {
                None => stack.push(c),
                Some(d) => {
                    if d == *stack.last().unwrap() {
                        stack.pop();
                    } else {
                        corrupted = true;
                        break;
                    }
                }
            }
        }
        if !corrupted {
            let mut tmp = 0;
            for c in stack.into_iter().rev() {
                tmp *= 5;
                match c {
                    '(' => tmp += 1,
                    '[' => tmp += 2,
                    '{' => tmp += 3,
                    '<' => tmp += 4,
                    _ => ()
                }
            }
            scores.push(tmp);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
