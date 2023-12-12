use std::collections::HashMap;
use std::str::FromStr;
use std::thread;

#[derive(Clone)]
enum Step {
    Left,
    Right,
}

impl From<char> for Step {
    fn from(value: char) -> Self {
        match value {
            'L' => Step::Left,
            'R' => Step::Right,
            _ => panic!("Unknown step"),
        }
    }
}

struct Map {
    steps: Vec<Step>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Map {
    fn steps_to(&self, target: Node) -> i32 {
        let mut current_node = &Node("AAA".to_string());
        let mut i = 0;
        let mut steps = self.steps.iter().cycle();

        while current_node != &target {
            let next_nodes = self.nodes.get(&current_node).unwrap();
            current_node = match steps.next().unwrap() {
                Step::Left => &next_nodes.0,
                Step::Right => &next_nodes.1,
            };
            i += 1;
        }

        i
    }

    fn ghost_steps_to(&self, target: char) -> i64 {
        let current_nodes = self.nodes.keys().cloned()
            .filter(|node| node.ends_with('A')).collect::<Vec<_>>();

        let mut handles = Vec::new();
        let mut results = Vec::new();

        for node in current_nodes {
            let nodes = self.nodes.clone(); // Clone only the necessary part
            let steps = self.steps.iter().cloned().collect::<Vec<_>>(); // Cycle and clone the iterator
            let target = target.clone();
            let handle = thread::spawn(move || {
                let mut i = 0;
                let mut current_node = node;
                while !current_node.ends_with(target) {
                    let next_nodes = nodes.get(&current_node).unwrap();
                    current_node = match steps[i % steps.len()] {
                        Step::Left => next_nodes.0.clone(),
                        Step::Right => next_nodes.1.clone(),
                    };
                    i += 1;
                }

                i
            });
            handles.push(handle);
        }

        for handle in handles {
            results.push(handle.join().unwrap());
        }

        results.iter()
            .fold(1, |a, b| lcm(a, *b)) as i64
            // .max().copied().unwrap_or_default() as i32
    }

}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let steps = lines.next()
            .map(|s| s.chars().map(|c| Step::from(c)).collect::<Vec<Step>>()).unwrap();

        lines.next();

        let mut nodes: HashMap<Node, (Node, Node)> = HashMap::new();
        for line in lines {
            nodes.insert(
                Node(line[..3].to_string()),
                (
                    Node(line[7..10].to_string()),
                    Node(line[12..15].to_string())
                ));
        }

        Ok(Map {
            steps,
            nodes,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node(String);

impl Node {
    fn ends_with(&self, char: char) -> bool {
        self.0.ends_with(char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_part() {
        let input = include_str!("../../input/day_08/input.txt");

        let map = Map::from_str(input).unwrap();

        assert_eq!(map.steps_to(Node("ZZZ".to_string())), 16897);
    }

    #[test]
    fn second_part() {
        let input = include_str!("../../input/day_08/input.txt");

        let map = Map::from_str(input).unwrap();

        assert_eq!(map.ghost_steps_to('Z'), 16563603485021);
    }

    #[test]
    fn example() {
        let input = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)\n\
        ";

        let map = Map::from_str(input).unwrap();

        assert_eq!(map.ghost_steps_to('Z'), 6);
    }

    #[test]
    fn node_a() {
        let node = Node("AAA".to_string());

        assert_eq!(node.ends_with('A'), true);
        assert_eq!(node.ends_with('Z'), false);
    }

    #[test]
    fn node_z() {
        let node = Node("ZZZ".to_string());

        assert_eq!(node.ends_with('A'), false);
        assert_eq!(node.ends_with('Z'), true);
    }
}
