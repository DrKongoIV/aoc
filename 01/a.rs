use std::io;
use std::io::BufRead;

fn main() {
    let mut counter = 0;
    let mut last = 0;
    let mut first = true;
    for line in io::stdin().lock().lines() {
        let num = line.unwrap().parse::<u32>().unwrap();
        if !first && last < num {
            counter = counter + 1;
        }
        last = num;
        first = false;
    }
    println!("{}", counter);
}
