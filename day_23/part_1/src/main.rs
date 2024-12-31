use std::collections::{HashMap, BTreeSet};
use std::collections::HashSet;

//  const INPUT: &str = include_str!("test_input.txt");
const INPUT: &str = include_str!("real_input.txt"); 


fn bron_kerbosch_all_cliques(
    r: &BTreeSet<String>,
    p: &mut BTreeSet<String>,
    x: &mut BTreeSet<String>,
    g: &HashMap<String, BTreeSet<String>>,
    cliques: &mut Vec<Vec<String>>,
) {
    if p.is_empty() {
        if r.len() > 1 {
            let mut clique: Vec<String> = r.iter().cloned().collect();
            // clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Iterate over all vertices in P (instead of using a pivot)
    let v = p.iter().next().cloned().unwrap();
    let neighbors_v = g.get(&v).cloned().unwrap_or_default();
    
    let mut new_r = r.clone();
    new_r.insert(v.clone());

    let mut new_p = p.intersection(&neighbors_v).cloned().collect::<BTreeSet<String>>();
    let mut new_x = x.intersection(&neighbors_v).cloned().collect::<BTreeSet<String>>();

    bron_kerbosch_all_cliques(&new_r, &mut new_p, &mut new_x, g, cliques);

    // Move v from P to X
    p.remove(&v);
    x.insert(v);

    // Recurse with the remaining vertices in P (after updating p and x)
    if !p.is_empty() {
        bron_kerbosch_all_cliques(r, p, x, g, cliques);
    }
}

fn main() {
    // Define the input edges
    let mut input = vec![("a", "b")];

    input.clear();

    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split("-").collect();
        let comp_1 = parts[0];
        let comp_2 = parts[1];
        input.push((comp_1, comp_2));
    }

    // Build the graph as an adjacency list
    let mut graph: HashMap<String, BTreeSet<String>> = HashMap::new();
    for (src, dest) in input.iter() {
        graph
            .entry(src.to_string())
            .or_insert_with(BTreeSet::new)
            .insert(dest.to_string());
        graph
            .entry(dest.to_string())
            .or_insert_with(BTreeSet::new)
            .insert(src.to_string());
    }

    // Initialize R, P, X
    let r: BTreeSet<String> = BTreeSet::new();
    let mut p: BTreeSet<String> = graph.keys().cloned().collect();
    let mut x: BTreeSet<String> = BTreeSet::new();

    // Collect cliques
    let mut cliques: Vec<Vec<String>> = Vec::new();
    bron_kerbosch_all_cliques(&r, &mut p, &mut x, &graph, &mut cliques);

    // Sort the cliques for consistent output
    let mut sorted_cliques = cliques.clone();
    sorted_cliques.sort();

    // println!("{:?}", sorted_cliques);

    // println!("{}", sorted_cliques.len());

    // Print each clique
    let mut total = 0;
    let mut new_set: Vec<HashSet<String>> = Vec::new();

    for clique in sorted_cliques {
        if clique.len() == 3 {
            let s = clique.join(", ");
            let parts: Vec<&str> = s.split(',').map(|l| l.trim()).collect();
            let mut new_vec: HashSet<String> = HashSet::new();

            let mut keep = false;
            for p in &parts {
                if p.contains("t") {
                    keep = true;
                }
                new_vec.insert(p.to_string());
            }
            if keep {
                if !exists_already(new_vec.clone(), new_set.clone()) {
                    new_set.push(new_vec);
                }
            }
        }
    }

    println!("{}", new_set.len());
}

fn exists_already(looking: HashSet<String>, orig: Vec<HashSet<String>>) -> bool {
    for i in orig {
        if i == looking {
            return true;
        }
    }

    return false;
}