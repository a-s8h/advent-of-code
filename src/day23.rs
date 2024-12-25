use std::collections::{HashMap, HashSet};

type ComputerName = String;
type Network = HashMap<ComputerName, HashSet<ComputerName>>;
type Triangle = HashSet<ComputerName>;

/// Finds how many sets of three inter-connected computers contain at least
/// one computer with a name starting with 't'
pub fn part1(input: &str) -> usize {
    let network = parse_network(input);
    find_triangles(&network)
        .into_iter()
        .filter(contains_t_computer)
        .count()
}

pub fn part2(input: &str) -> String {
    let network = parse_network(input);
    let max_clique = find_max_clique(&network);
    format_clique(&max_clique)
}

/// Parses input into an undirected graph representation
fn parse_network(input: &str) -> Network {
    let mut network = Network::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').expect("Invalid connection format");
        network
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        network
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    network
}

/// Returns true if any computer in the triangle starts with 't'
fn contains_t_computer(triangle: &Triangle) -> bool {
    triangle.iter().any(|name| name.starts_with('t'))
}

/// Checks if three computers form a complete triangle in the network
fn is_triangle(network: &Network, a: &str, b: &str, c: &str) -> bool {
    network[a].contains(b) && network[a].contains(c) && network[b].contains(c)
}

/// Finds all sets of three inter-connected computers
fn find_triangles(network: &Network) -> Vec<Triangle> {
    let computers: Vec<_> = network.keys().map(String::as_str).collect();
    let mut triangles = Vec::new();

    for (i, &a) in computers.iter().enumerate() {
        for &b in &computers[i + 1..] {
            if !network[a].contains(b) {
                continue;
            }
            for &c in &computers[computers.iter().position(|&x| x == b).unwrap() + 1..] {
                if is_triangle(network, a, b, c) {
                    let triangle = [a, b, c].iter().map(|&s| s.to_string()).collect();
                    triangles.push(triangle);
                }
            }
        }
    }

    triangles
}

/// Format clique members into password string (sorted, comma-separated)
fn format_clique(clique: &HashSet<ComputerName>) -> String {
    let mut computers: Vec<_> = clique.iter().collect();
    computers.sort();
    computers
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

/// Find the largest set of fully interconnected computers
fn find_max_clique(network: &Network) -> Triangle {
    let mut max_clique = Triangle::new();
    let mut compsub = Triangle::new();
    let mut candidates: HashSet<_> = network.keys().cloned().collect();
    let mut not = Triangle::new();

    bron_kerbosch(
        network,
        &mut compsub,
        &mut candidates,
        &mut not,
        &mut max_clique,
    );

    max_clique
}

fn bron_kerbosch(
    network: &Network,
    compsub: &mut Triangle,
    candidates: &mut Triangle,
    not: &mut Triangle,
    max_clique: &mut Triangle,
) {
    if candidates.is_empty() && not.is_empty() {
        if compsub.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(compsub.iter().cloned());
        }
        return;
    }

    // Choose pivot from candidates ∪ not that maximizes |P ∩ N(u)|
    let pivot = candidates
        .iter()
        .chain(not.iter())
        .max_by_key(|&v| candidates.intersection(&network[v]).count())
        .unwrap()
        .clone();

    let ext_candidates: Vec<_> = candidates.difference(&network[&pivot]).cloned().collect();

    for v in ext_candidates {
        let v_neighbors = &network[&v];

        // Add vertex to compsub
        compsub.insert(v.clone());

        // Create new candidates and not sets
        let new_candidates: Triangle = candidates.intersection(v_neighbors).cloned().collect();
        let new_not: Triangle = not.intersection(v_neighbors).cloned().collect();

        // Recursive call
        bron_kerbosch(
            network,
            compsub,
            &mut new_candidates.clone(),
            &mut new_not.clone(),
            max_clique,
        );

        // Move v from candidates to not
        compsub.remove(&v);
        candidates.remove(&v);
        not.insert(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_network_parsing() {
        let network = parse_network("a-b\nb-c");
        assert!(network["a"].contains("b"));
        assert!(network["b"].contains("a"));
        assert!(network["b"].contains("c"));
        assert!(network["c"].contains("b"));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day23.txt").unwrap();
        assert_eq!(part1(&input), 1240);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day23.txt").unwrap();
        assert_eq!(part2(&input), "am,aq,by,ge,gf,ie,mr,mt,rw,sn,te,yi,zb");
    }
}
