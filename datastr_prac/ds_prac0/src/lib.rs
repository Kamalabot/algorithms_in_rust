#[allow(unused_variables)]
use std::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            val: value,
            next: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Binode {
    pub val: i32,
    pub left: Option<Box<Binode>>,
    pub right: Option<Box<Binode>>,
}

pub fn stack_traverse(root: Option<Box<Node>>) -> String {
    // The type that we are choosing to send in
    // will decide how they behave inside the function
    let mut output = String::new();
    let mut curr = root;

    while let Some(node) = curr {
        write!(&mut output, "{} -->", node.val).unwrap();
        curr = node.next;
    }
    println!("{}", output);
    output
}

pub fn fast_slow(root: Node) -> bool {
    true
}

pub fn depth_first_tt(root: Binode) -> String {
    "Your traversal algo here".to_owned()
}

pub fn binary_first_tt(root: Binode) -> String {
    "Your traversal algo here".to_owned()
}

pub fn inplace_reversal(root: Node) -> String {
    "Your traversal algo here".to_owned()
}

pub fn make_list(in_list: Vec<i32>) -> Option<Box<Node>> {
    let mut curr = None;

    for &val in in_list.iter().rev() {
        let mut node = Box::new(Node::new(val));
        node.next = curr;
        curr = Some(node);
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_list_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        println!("Starting list making");
        let root = make_list(in_list).unwrap();
        // if the in_list is moved in to make_list, it cannot be used again
        assert_eq!(root.val, 5);
    }
    #[test]
    fn stack_traverse_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        println!("List is made");
        let root = make_list(in_list).unwrap();
        // if the in_list is moved in to make_list, it cannot be used again
        println!("Testing the traversal");
        assert_eq!(
            stack_traverse(Some(root)),
            "5 -->7 -->6 -->2 -->3 -->9 -->10 -->12 -->".to_owned()
        );
    }
}