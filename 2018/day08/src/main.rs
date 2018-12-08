use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Couldn't read file");
    let input: Vec<usize> = input
        .split_whitespace()
        .filter_map(|d| d.parse::<usize>().ok())
        .collect();

    let (node, _) = Node::parse(input);

    println!("Part one: {}", node.metadata_sum());
    println!("Part two: {}", node.value());
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(input: Vec<usize>) -> (Node, usize) {
        let n_children = input.get(0).unwrap();
        let n_metadata = input.get(1).unwrap();

        let mut start = 2;
        let mut children = Vec::new();
        for _ in 0..*n_children {
            let (child, size) = Node::parse(input[start..].to_vec());
            children.push(child);
            start += size;
        }

        let metadata = input[start..start + n_metadata].to_vec();
        (Node { children, metadata }, start + n_metadata)
    }

    fn metadata_sum(&self) -> usize {
        let child_sum: usize = self.children.iter().map(|c| c.metadata_sum()).sum();
        let meta_sum: usize = self.metadata.iter().sum();
        child_sum + meta_sum
    }

    fn value(&self) -> usize {
        if self.children.len() == 0 {
            return self.metadata.iter().sum();
        }

        self.metadata
            .iter()
            .map(|m| self.children.get(m - 1).map_or(0, |c| c.value()))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn node_parsing_test() {
        assert_eq!(
            Node::parse(vec![0, 1, 99]),
            (
                Node {
                    children: Vec::new(),
                    metadata: vec![99]
                },
                3
            )
        );
    }

    #[test]
    fn _node_with_children_test() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let d = Node {
            children: vec![],
            metadata: vec![99],
        };
        let c = Node {
            children: vec![d],
            metadata: vec![2],
        };
        let b = Node {
            children: vec![],
            metadata: vec![10, 11, 12],
        };
        let a = Node {
            children: vec![b, c],
            metadata: vec![1, 1, 2],
        };
        let (node, _) = Node::parse(input);
        assert_eq!(node, a);
    }

    #[test]
    fn metadata_sum_test() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let (node, _) = Node::parse(input);
        assert_eq!(node.metadata_sum(), 138);
    }

    #[test]
    fn childless_node_value_test() {
        let b = Node {
            children: vec![],
            metadata: vec![10, 11, 12],
        };
        assert_eq!(b.value(), 33);
    }

    #[test]
    fn node_value_with_children_test() {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let (node, _) = Node::parse(input);
        assert_eq!(node.value(), 66);
    }
}
