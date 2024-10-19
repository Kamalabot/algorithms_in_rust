#![allow(unused_variables)]
use std::fmt::Write;
//Create a Node struct with val and next
//impl a new fn
//derive PartialEq, Eq with Debug
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
// Implement Binode struct with val, left & right
// PartialEq, Eq is required
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Binode {
    pub val: i32,
    pub left: Option<Box<Binode>>,
    pub right: Option<Box<Binode>>,
}

impl Binode {
    fn new(val: i32) -> Self {
        Binode {
            val,
            left: None,
            right: None,
        }
    }
}
// Write a function that traverses stack and prints
pub fn stack_traverse(root: Option<Box<Node>>) -> String {
    // The type that we are choosing to send in
    // will decide how they behave inside the function
    // declare empty output string, and make root node as curr
    // use while let Some constuct
    // write! node.val to output and move curr to next
    let mut output = String::new();
    let mut curr = root;

    while let Some(node) = curr {
        write!(&mut output, "{} -->", node.val).unwrap();
        curr = node.next;
    }
    println!("{}", output);
    output
}
// implement the fast slow to capture the loops
pub fn fast_slow(root: Option<&Box<Node>>) -> bool {
    // assign root to both fast and slow
    // use while let (Some, Some) construct on fast, slow
    // inside while assign slow to slow.next.as_ref
    // use if let some construct to assign fast to root.next.next
    // if reached end w/o loop, then return false
    // else check slow == false and return true
    // while loop ends then return false
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
// finding the middle of the list
// if fast reaches end, then slow is at the mid
pub fn list_mid(root: Option<&Box<Node>>) -> Option<i32> {
    // assign root to slow and fast
    // use while let (some, some) construct
    // inside while assign slow to root.next
    // use if let Some to assign root.next.nxt to fas
    // check if fast is none, then return slow.map(|node| node.val)
    // return the value of slow node
    let mut slow = root;
    let mut fast = root;
    while let (Some(f), Some(s)) = (fast, slow) {
        slow = s.next.as_ref();
        // in order to reach the fast.next.next
        // need to understand the below syntax,
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

pub fn inplace_reversal(root: Option<Box<Node>>) -> Option<Box<Node>> {
    // make prev as none and current as root
    // use while let some construct on current
    // take the next node, using curr_node.next
    // make curr_node.next as prev
    // assign Some(curr_node) to prev
    // assign next to current
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
    // if m is n then return head
    if m == n {
        return head;
    }
    // create a dummy node which is Option<Box<Node>> with 0 val and head as next
    // assign dummy.as_mut() to prev
    // iterate on 0..m - 1 move prev by m nodes,
    // take the prev.next as mut and assign to current
    // do the same with current and assign to next
    // iterate between m..n
    // do the above with next and assign it to temp
    // assign current to next's next (this is tricky)
    // assign next to current
    // assign temp to next
    // after iteration, assign next to prev.next.next
    // after iteration, assign current to prev.next
    // return dummy's next
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
    // this can be implement as from_vec fn
    // declare curr as None
    // use vector iter in reverse
    // create new boxed nodes
    // assign curr to created node's next
    // assign created node to curr
    // return curr after iteration
    let mut curr = None;

    for &val in in_list.iter().rev() {
        let mut node = Box::new(Node::new(val));
        node.next = curr;
        curr = Some(node);
    }
    curr
}

pub fn make_tree(in_list: Vec<i32>) -> Option<Box<Binode>> {
    // traverses and attaches data to the tree nodes
    // check if vector is empty then return None
    // assign a new boxed binode to curr
    // assign a new vec wth curr to queue
    // declare mut idx to 1
    // start while loop till idx < in_list.len
    // in side the while loop, remove the 0th node with if let some construct
    // check if idx < in_list.len() and assign idx value to left node
    // push the created left node into queue
    // increment idx and do the same for right node and increment idx
    // return curr
    if in_list.is_empty() {
        return None;
    }
    let curr = Some(Box::new(Binode::new(in_list[0])));
    let mut queue = vec![curr.clone()];

    let mut idx = 1;
    while idx < in_list.len() {
        // pop the first element from the queue
        if let Some(mut node) = queue.remove(0) {
            // attach the left and right node to the curr node
            if idx < in_list.len() {
                node.left = Some(Box::new(Binode::new(in_list[idx])));
                queue.push(node.left.clone())
            }
            idx += 1;
            if idx < in_list.len() {
                node.right = Some(Box::new(Binode::new(in_list[idx])));
                queue.push(node.right.clone())
            }
            idx += 1;
        }
    }
    curr
}

pub fn depth_first_tt(root: &Option<Box<Binode>>, result: &mut Vec<i32>) {
    // use if let some on root
    // call self on n.left, along with result
    // push n.val into result
    // call self on n.right along with result
    if let Some(n) = root {
        depth_first_tt(&n.left, result);
        result.push(n.val);
        depth_first_tt(&n.right, result);
    }
}

use std::collections::VecDeque;

pub fn binary_first_tt(root: &Option<Box<Binode>>) -> Vec<i32> {
    // create result vector and queue VecDeque
    // push root node into queue using if let constuct
    // start a while let some constuct on queue.pop_front
    // push the value into result
    // do if let constuct on left and right and push_back on queue
    // after while construct return result
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(node.clone());
    }
    while let Some(node) = queue.pop_front() {
        result.push(node.val);

        if let Some(left) = &node.left {
            queue.push_back(left.clone());
        }
        if let Some(right) = &node.right {
            queue.push_back(right.clone());
        }
    }
    result
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
