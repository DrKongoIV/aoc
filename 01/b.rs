use std::io;
use std::io::BufRead;

fn main() {
    let mut counter = 0;
    let mut nums: [u32; 3] = [0; 3];
    let mut i = 0;
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        let new = line.unwrap().parse::<u32>().unwrap();
        nums[i % 3] = new;
        let new_sum = nums.iter().sum();
        if i >= 3 && new_sum > sum {
            counter = counter + 1;
        }
        sum = new_sum;
        i = i + 1;
    }
    println!("{}", counter);
}
