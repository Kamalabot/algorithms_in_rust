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

pub fn fast_slow(root: Option<&Box<Node>>) -> bool {
    let mut slow = root;
    let mut fast = root;
    while let (Some(f), Some(s)) = (fast, slow) {
        slow = s.next.as_ref();
        // in order to reach the fast.next.next
        // need to use if let as below
        if let Some(next_fast) = &f.next {
            fast = next_fast.next.as_ref();
        } else {
            return false;
        }
        if slow == fast {
            return true;
        }
    }
    false
}

pub fn list_mid(root: Option<&Box<Node>>) -> Option<i32> {
    let mut slow = root;
    let mut fast = root;
    while let (Some(f), Some(s)) = (fast, slow) {
        slow = s.next.as_ref();
        // in order to reach the fast.next.next
        // need to understand the below syntax
        fast = if let Some(next_fast) = &f.next {
            next_fast.next.as_ref()
        } else {
            None
        };
        if fast.is_none() {
            return slow.map(|node| node.val);
        }
    }
    None
}

pub fn depth_first_tt(root: Binode) -> String {
    "Your traversal algo here".to_owned()
}

pub fn binary_first_tt(root: Binode) -> String {
    "Your traversal algo here".to_owned()
}

pub fn inplace_reversal(root: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev = None;
    let mut current = root;

    while let Some(mut curr_node) = current {
        let next = curr_node.next.take();
        curr_node.next = prev;
        prev = Some(curr_node);
        current = next;
    }
    prev
}

pub fn reverse_between(mut head: Option<Box<Node>>, m: usize, n: usize) -> Option<Box<Node>> {
    if m == n {
        return head;
    }

    let mut dummy = Some(Box::new(Node { val: 0, next: head }));
    let mut prev = dummy.as_mut();

    for _ in 0..m - 1 {
        prev = prev.unwrap().next.as_mut();
    }

    let mut current = prev.as_mut().unwrap().next.take();
    let mut next = current.as_mut().unwrap().next.take();

    for _ in m..n {
        let temp = next.as_mut().unwrap().next.take();
        next.as_mut().unwrap().next = current;
        current = next;
        next = temp;
    }

    prev.as_mut().unwrap().next.as_mut().unwrap().next = next;
    prev.as_mut().unwrap().next = current;

    dummy.unwrap().next
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

    #[test]
    fn fast_slow_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        println!("List is made");
        let root = make_list(in_list).unwrap();
        // if the in_list is moved in to make_list, it cannot be used again
        println!("Testing two pointer");
        // let box_opt_root = Some(Box::new(root));
        // we have to get no cycle till here
        assert!(!fast_slow(Some(&root)));
        // TODO need to think of creating a cycle below
    }
}
