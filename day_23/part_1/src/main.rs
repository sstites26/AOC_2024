use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut comp_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in INPUT.lines() {
       parse_line(line, &mut comp_map);
    }

    for (key, value) in comp_map.clone().into_iter() {
        println!("Computer {} : {:?}", key, value);

        check_nodes(comp_map.clone(), &value.clone());
        break;
    }
}

fn check_nodes(comp_map: HashMap<String, HashSet<String>>, nodes: &HashSet<String>) {
    let mut matching = 0;
    for node in nodes {
        let main_node = node.clone();
        for node2 in nodes {
            let nn = node2.clone();
            if *nn != main_node {
                if comp_map.get(&nn).expect("REASON").contains(&main_node) {
                    matching += 1;
                }
            }
        }
    }

    println!("Match: {}", matching);
}

fn parse_line(line: &str, comp_map: &mut HashMap<String, HashSet<String>>) {    
    let parts: Vec<&str> = line.split("-").collect();

    let comp_1 = parts[0];
    let comp_2 = parts[1];

    if comp_map.contains_key(comp_1) {
        let connections = comp_map.get_mut(comp_1).expect("REASON");
        connections.insert(comp_2.to_string());
    } else {
        let mut connections : HashSet<String> = HashSet::new();
        connections.insert(comp_2.to_string());
        comp_map.insert(comp_1.to_string(), connections);
    }
    
    if comp_map.contains_key(comp_2) {
        let connections = comp_map.get_mut(comp_2).expect("REASON");
        connections.insert(comp_1.to_string());
    } else {
        let mut connections : HashSet<String> = HashSet::new();
        connections.insert(comp_1.to_string());
        comp_map.insert(comp_2.to_string(), connections);
    }
}