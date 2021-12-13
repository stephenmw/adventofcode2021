use graph_parser::Graph;
use std::collections::HashSet;

pub fn problem1(input: &str) -> String {
    let g = graph_parser::parse(input).unwrap().1;
    let ans = find_paths(&g).len();
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let g = graph_parser::parse(input).unwrap().1;
    let ans = find_paths2(&g).len();
    format!("{}", ans)
}

// Returns a list of paths
fn find_paths(g: &Graph) -> Vec<Vec<usize>> {
    let mut frontier: Vec<(usize, Vec<usize>, HashSet<usize>)> = Vec::new();
    let start = g.get_id_by_name("start").expect("no start node?");
    frontier.push((start, Vec::new(), HashSet::new()));

    let mut ret = Vec::new();

    while let Some((cur, mut path, mut seen)) = frontier.pop() {
        let node = g.get_node(cur).unwrap();

        if !node.is_big && !seen.insert(cur) {
            continue;
        }
        path.push(cur);

        if node.name == "end" {
            ret.push(path);
            continue;
        }

        frontier.extend(
            node.neighbors
                .iter()
                .map(|&x| (x, path.clone(), seen.clone())),
        )
    }

    ret
}

// Returns a list of paths
fn find_paths2(g: &Graph) -> Vec<Vec<usize>> {
    let mut frontier: Vec<(usize, Vec<usize>, HashSet<usize>, bool)> = Vec::new();
    let start = g.get_id_by_name("start").expect("no start node?");
    frontier.push((start, Vec::new(), HashSet::new(), false));

    let mut ret = Vec::new();

    while let Some((cur, mut path, mut seen, mut pass_used)) = frontier.pop() {
        let node = g.get_node(cur).unwrap();

        let need_pass = !node.is_big && !seen.insert(cur);
        if need_pass {
            if pass_used {
                continue;
            }
            if node.name == "start" {
                // may not visit start a second time
                continue;
            }
            pass_used = true;
        }
        path.push(cur);

        if node.name == "end" {
            ret.push(path);
            continue;
        }

        frontier.extend(
            node.neighbors
                .iter()
                .map(|&x| (x, path.clone(), seen.clone(), pass_used)),
        )
    }

    ret
}

mod graph_parser {
    use crate::lib::combinators::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default)]
    pub struct Graph {
        nodes: Vec<Node>,
        node_ids: HashMap<String, usize>,
    }

    impl Graph {
        fn new() -> Self {
            Graph::default()
        }

        pub fn get_id_by_name(&self, name: &str) -> Option<usize> {
            self.node_ids.get(name).cloned()
        }

        fn get_id_by_name_or_insert(&mut self, name: &str) -> usize {
            if let Some(&node_id) = self.node_ids.get(name) {
                return node_id;
            }

            let new_id = self.nodes.len();
            self.nodes.push(Node::new(new_id, name.to_owned()));
            self.node_ids.insert(name.to_owned(), new_id);

            new_id
        }

        pub fn get_node(&self, id: usize) -> Option<&Node> {
            self.nodes.get(id)
        }

        fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
            self.nodes.get_mut(id)
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Node {
        pub id: usize,
        pub name: String,
        pub is_big: bool,
        pub neighbors: Vec<usize>,
    }

    impl Node {
        fn new(id: usize, name: String) -> Self {
            let is_big = name
                .chars()
                .next()
                .map(|x| x.is_uppercase())
                .unwrap_or(false);

            Node {
                id: id,
                name: name,
                is_big: is_big,
                neighbors: Vec::new(),
            }
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Graph> {
        let node_name = || verify(take_while(is_alphabetic), |x: &str| !x.is_empty());
        let edge = separated_pair(node_name(), tag("-"), node_name());
        let mut parser = separated_list1(line_ending, edge);
        let (r, edges) = parser(input)?;

        let mut g = Graph::new();
        for (node_a, node_b) in edges {
            let a_id = g.get_id_by_name_or_insert(node_a);
            let b_id = g.get_id_by_name_or_insert(node_b);

            g.get_node_mut(a_id).unwrap().neighbors.push(b_id);
            g.get_node_mut(b_id).unwrap().neighbors.push(a_id);
        }

        Ok((r, g))
    }
}
