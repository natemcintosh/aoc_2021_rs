use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Node<'a> {
    SmallNode {
        name: &'a str,
        has_been_visited: bool,
    },
    BigNode {
        name: &'a str,
    },
}

fn parse_node(n: &str) -> Node {
    if n.chars()
        .next()
        .expect("Could not find any characters on this node")
        .is_lowercase()
    {
        Node::SmallNode {
            name: n,
            has_been_visited: false,
        }
    } else {
        Node::BigNode { name: n }
    }
}

fn parse_input(input: &str) -> (HashSet<Node>, HashMap<&str, HashSet<&str>>) {
    let node_strs: HashSet<&str> = input.lines().flat_map(|l| l.split('-')).collect();
    let nodes: HashSet<Node> = node_strs.iter().map(|&n| parse_node(n)).collect();
    let mut adjacency_list: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (s1, s2) in input
        .lines()
        .map(|l| l.split_once('-').expect("Could not split on '-'"))
    {
        let adj1 = adjacency_list.entry(s1).or_insert_with(HashSet::new);
        adj1.insert(s2);

        let adj2 = adjacency_list.entry(s2).or_insert_with(HashSet::new);
        adj2.insert(s1);
    }
    (nodes, adjacency_list)
}

fn find_node_by_name<'a>(nodes: &'a HashSet<Node>, name_str: &'a str) -> Node<'a> {
    nodes
        .iter()
        .find(|&n| match n {
            Node::BigNode { name } | Node::SmallNode { name, .. } => *name == name_str,
        })
        .expect("Could not find node in set of nodes")
        .clone()
}

fn find_all_paths<'a>(
    nodes: &'a HashSet<Node>,
    adjacency_list: &'a HashMap<&str, HashSet<&str>>,
) -> Vec<Vec<&'a str>> {
    let start_node = find_node_by_name(nodes, "start");

    let mut frontier = VecDeque::new();
    frontier.push_back((start_node, Vec::<&str>::new()));

    let mut results = Vec::new();

    while let Some(p) = frontier.pop_front() {
        let n = p.0;
        let mut path_here = p.1;
        if let Node::SmallNode { name, .. } = n {
            if name == "end" {
                // Add "end" to the path here
                path_here.push("end");
                results.push(path_here);
                continue;
            }

            // If this is a small node, and it is already in the `path_here`, then don't
            // continue processing
            if path_here.contains(&name) {
                continue;
            }
        }

        let name = match n {
            Node::SmallNode { name, .. } => name,
            Node::BigNode { name } => name,
        };
        // Add all the neighbors to the frontier, adding current node to the `path_here`
        for &nbr in adjacency_list
            .get(name)
            .expect("Could not find a node in the adjacency list")
        {
            let mut path_to_nbr = path_here.clone();
            path_to_nbr.push(name);
            let node = find_node_by_name(nodes, nbr);
            frontier.push_back((node, path_to_nbr));
        }
    }

    results
}

fn main() {
    let setup_time = std::time::Instant::now();

    let input_str =
        std::fs::read_to_string("input/day12.txt").expect("Failed to read day 12 input");
    let (nodes, adjacency_list) = parse_input(&input_str);
    println!("Setup took {:.6} µs", setup_time.elapsed().as_micros());

    // Part 1
    let part1_time = std::time::Instant::now();
    let all_paths = find_all_paths(&nodes, &adjacency_list);
    let part1_result = all_paths.len();
    println!("Part 1 took {:.6} µs", part1_time.elapsed().as_micros());

    // Part 2
    let part2_time = std::time::Instant::now();
    println!("Part 2 took {:.6} µs", part2_time.elapsed().as_micros());

    println!();
    println!("Part 1 result: {}", part1_result);
    // println!("Part 2 result: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_1() {
        let input_str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let (got_nodes, adjacency_list) = parse_input(input_str);

        let expected_adjacencies = HashMap::from([
            ("start", HashSet::from(["A", "b"])),
            ("b", HashSet::from(["start", "A", "d", "end"])),
            ("A", HashSet::from(["start", "c", "end", "b"])),
            ("c", HashSet::from(["A"])),
            ("d", HashSet::from(["b"])),
            ("end", HashSet::from(["A", "b"])),
        ]);
        assert_eq!(expected_adjacencies, adjacency_list);

        let expected_nodes = HashSet::from([
            Node::SmallNode {
                name: "start",
                has_been_visited: false,
            },
            Node::SmallNode {
                name: "b",
                has_been_visited: false,
            },
            Node::BigNode { name: "A" },
            Node::SmallNode {
                name: "c",
                has_been_visited: false,
            },
            Node::SmallNode {
                name: "d",
                has_been_visited: false,
            },
            Node::SmallNode {
                name: "end",
                has_been_visited: false,
            },
        ]);

        assert_eq!(expected_nodes, got_nodes)
    }

    #[test]
    fn test_parse_input_2() {
        let input_str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let (got_nodes, adjacency_list) = parse_input(input_str);

        let expected_adjacencies = HashMap::from([
            ("start", HashSet::from(["HN", "kj", "dc"])),
            ("HN", HashSet::from(["start", "dc", "end", "kj"])),
            ("dc", HashSet::from(["start", "HN", "LN", "kj", "end"])),
            ("LN", HashSet::from(["dc"])),
            ("kj", HashSet::from(["start", "sa", "HN", "dc"])),
            ("sa", HashSet::from(["kj"])),
            ("end", HashSet::from(["HN", "dc"])),
        ]);
        assert_eq!(expected_adjacencies, adjacency_list);

        let expected_nodes = HashSet::from([
            Node::SmallNode {
                name: "start",
                has_been_visited: false,
            },
            Node::BigNode { name: "HN" },
            Node::SmallNode {
                name: "dc",
                has_been_visited: false,
            },
            Node::BigNode { name: "LN" },
            Node::SmallNode {
                name: "kj",
                has_been_visited: false,
            },
            Node::SmallNode {
                name: "sa",
                has_been_visited: false,
            },
            Node::SmallNode {
                name: "end",
                has_been_visited: false,
            },
        ]);

        assert_eq!(expected_nodes, got_nodes)
    }

    #[test]
    fn test_part1_1() {
        let input_str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let (nodes, adjacency_list) = parse_input(input_str);

        let got = find_all_paths(&nodes, &adjacency_list);
        assert_eq!(10, got.len())
    }

    #[test]
    fn test_part1_2() {
        let input_str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let (nodes, adjacency_list) = parse_input(input_str);

        let got = find_all_paths(&nodes, &adjacency_list);
        assert_eq!(19, got.len())
    }

    #[test]
    fn test_part1_3() {
        let input_str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let (nodes, adjacency_list) = parse_input(input_str);

        let got = find_all_paths(&nodes, &adjacency_list);
        assert_eq!(226, got.len())
    }
}
