use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    name_id: usize,
    is_big: bool,
    has_been_visited: bool,
}

fn parse_input(input: &str) {}

fn main() {
    println!("hello world");
}

#[test]
fn test_parse_input() {
//     let input_str = "start-A
// start-b
// A-c
// A-b
// b-d
// A-end
// b-end";
//     let (got_graph, name_mapping) = parse_input(input_str);

//     let expected_mapping = HashMap::from([
//         (1, "start"),
//         (2, "b"),
//         (3, "A"),
//         (4, "c"),
//         (5, "d"),
//         (6, "end"),
//     ]);
//     let expected_graph = UnGraph::<Node, ()>::from_edges([(
//         Node {
//             name_id: 1,
//             is_big: false,
//             has_been_visited: false,
//         },
//         Node {
//             name_id: 2,
//             is_big: false,
//             has_been_visited: false,
//         },
//     )]);

//     assert_eq!(expected_graph, got_graph);
}
