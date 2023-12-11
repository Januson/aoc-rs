use std::collections::HashMap;
use std::str::FromStr;

enum Step {
    Left, Right
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

#[derive(Debug, Eq, PartialEq, Hash)]
struct Node(String);

pub struct CircularVec<T> {
    items: Vec<T>,
    index: usize,
}

impl<T> CircularVec<T> {
    fn new(items: Vec<T>) -> Self {
        CircularVec {
            index: 0,
            items,
        }
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

}
