use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::algo::clique::maximal_cliques;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, Hash, PartialEq)]
struct CompNode {
    name: String,
    index: i32,
    neighbors: Vec<String>
}

const INPUT: &str = include_str!("test_input.txt");
// const INPUT: &str = include_str!("real_input.txt"); 

fn main() {
    let mut comp_map: HashMap<String, CompNode> = HashMap::new();

    let mut node_index = 0;

    for line in INPUT.lines() {
       parse_line(line.to_string(), &mut node_index, &mut comp_map);
    }
    
    for (key, value) in comp_map {
        println!("{:?}: {:?}", key, value);
    }

    println!("{}", node_index);
}

fn parse_line(line: String, node_index: &mut i32, comp_map: &mut HashMap<String, CompNode>) {    
    let parts: Vec<&str> = line.split("-").collect();

    let comp_1 = parts[0];
    let comp_2 = parts[1];

    if comp_map.contains_key(comp_1) {
        let mut node = comp_map.get_mut(comp_1);
        node.unwrap().neighbors.push(comp_2.to_string());
    } else {
        let mut neighbors : Vec<String> = Vec::new();
        neighbors.push(comp_2.to_string());
        let new_node = CompNode{name: comp_1.to_string(), index: *node_index, neighbors: neighbors};

        comp_map.insert(comp_1.to_string(), new_node);
        *node_index += 1;
    }

    if comp_map.contains_key(comp_2) {
        let mut node = comp_map.get_mut(comp_2);
        node.unwrap().neighbors.push(comp_1.to_string());
    } else {
        let mut neighbors : Vec<String> = Vec::new();
        neighbors.push(comp_1.to_string());
        let new_node = CompNode{name: comp_2.to_string(), index: *node_index, neighbors: neighbors};

        comp_map.insert(comp_2.to_string(), new_node);
        *node_index += 1;
    }

    // println!("{:?}", comp_map);


    // println!("{:?} {:?}", comp_1, comp_2);

    
}