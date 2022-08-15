use std::collections::{HashMap, HashSet};

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

fn main() {
    println!("hello world");
}

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
