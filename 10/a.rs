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
    let mut acc = 0;
    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let mut stack : Vec<char> = Vec::new();
        for c in line.chars() {
            let opening = pair(c);
            match opening {
                None => stack.push(c),
                Some(d) => {
                    if d == *stack.last().unwrap() {
                        stack.pop();
                    } else {
                        match c {
                            ')' => acc += 3,
                            ']' => acc += 57,
                            '}' => acc += 1197,
                            '>' => acc += 25137,
                            _ => ()
                        }
                        break;
                    }
                }
            }
        }
    }
    println!("{}", acc);
}
