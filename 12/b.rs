use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn count_paths(graph : &HashMap::<String, Vec<String>>, mut visited : HashSet<String>, v : String, mut duplicate : String) -> u32 {
    if v == "end" {
        return 1;
    }
    if v == "start" && !visited.is_empty() {
        return 0;
    }
    if v.clone().chars().all(|c| matches!(c, 'a'..='z')) && visited.contains(&v) {
        if duplicate == "".to_string() {
            duplicate = v.clone();
        } else {
            return 0;
        }
    }
    visited.insert(v.clone());
    let mut sum = 0;
    for w in &graph[&v] {
        sum += count_paths(graph, visited.clone(), w.to_string(), duplicate.clone());
    }
    return sum;
}

fn main() {
    let lines : Vec<_> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mut graph = HashMap::<String, Vec<String>>::new();
    for line in lines.iter().map(|l| l.split("-").map(|s| s.to_string()).collect::<Vec<String>>()) {
        if graph.contains_key(&line[0]) {
            graph.get_mut(&line[0]).unwrap().push(line[1].clone());
        } else {
            graph.insert(line[0].clone(), vec![line[1].clone()]);
        }
        if graph.contains_key(&line[1]) {
            graph.get_mut(&line[1]).unwrap().push(line[0].clone());
        } else {
            graph.insert(line[1].clone(), vec![line[0].clone()]);
        }
    }
    let visited = HashSet::<String>::new();
    let kek = count_paths(&graph, visited, "start".to_string(), "".to_string());
    println!("{}", kek);
}
