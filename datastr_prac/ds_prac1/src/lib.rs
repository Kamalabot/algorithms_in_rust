#![allow(unused_variables)]
use std::fmt::Write;
//Create a Node struct with val and next
//impl a new fn
//derive PartialEq, Eq with Debug
#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Node {
        Self { val, next: None }
    }
}

// Implement Binode struct with val, left & right
// PartialEq, Eq is required
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Binode {
    val: i32,
    right: Option<Box<Binode>>,
    left: Option<Box<Binode>>,
}

impl Binode {
    pub fn new(val: i32) -> Binode {
        Self {
            val,
            right: None,
            left: None,
        }
    }
}

// Write a function that traverses stack and prints
pub fn stack_traverse(root: Option<Box<Node>>) -> String {
    // The type that we are choosing to send in
    // will decide how they behave inside the function
    // declare empty output string, and make root node as curr
    let mut output = String::new();
    let mut curr = root;
    // use while let Some constuct
    while let Some(node) = curr.take() {
        // write! node.val to output and move curr to next
        write!(&mut output, "{} -->", node.val);
        curr = node.next;
    }
    output
}
// implement the fast slow to capture the loops
pub fn fast_slow(root: Option<&Box<Node>>) -> bool {
    // assign root to both fast and slow
    let (mut fast, mut slow) = (root, root);
    // use while let (Some, Some) construct on fast, slow
    while let (Some(f), Some(s)) = (fast, slow) {
        // inside while assign slow to slow.next.as_ref
        slow = s.next.as_ref();
        // use if let some construct to assign fast to root.next.next
        if let Some(next_fast) = &f.next {
            fast = next_fast.next.as_ref()
        } else {
            return false;
        }
        if slow == fast {
            return true;
        }
    }
    // if reached end w/o loop, then return false
    // else check slow == false and return true
    // while loop ends then return false

    false
}
// finding the middle of the list
// if fast reaches end, then slow is at the mid
pub fn list_mid(root: Option<&Box<Node>>) -> Option<i32> {
    // assign root to slow and fast
    let (mut fast, mut slow) = (root, root);
    // use while let (some, some) construct
    while let (Some(f), Some(s)) = (fast, slow) {
        // inside while assign slow to root.next
        slow = s.next.as_ref();
        // use if let Some to assign root.next.nxt to fas
        if let Some(next_fast) = &f.next {
            // check if fast is none, then return slow.map(|node| node.val)
            next_fast.next.as_ref()
        } else {
            None
        };
        if fast.is_none() {
            // need to return the value
            return slow.map(|node| node.val);
        }
    }
    // return the value of slow node
    None
}

pub fn inplace_reversal(root: Option<Box<Node>>) -> Option<Box<Node>> {
    // make prev as none and current as root
    let mut prev = None;
    let mut curr = root;
    // use while let some construct on current
    while let Some(mut node) = curr {
        // take the next node, using curr_node.next
        let next = node.next.take();
        // make curr_node.next as prev
        node.next = prev;
        // assign Some(curr_node) to prev
        prev = Some(node);
        // assign next to current
        curr = next
    }
    prev
}

pub fn reverse_between(mut head: Option<Box<Node>>, m: usize, n: usize) -> Option<Box<Node>> {
    // if m is n then return head
    if m == n {
        return head;
    }
    // create a dummy node which is Option<Box<Node>> with 0 val and head as next
    let mut dummy = Some(Box::new(Node { val: 0, next: head }));
    // assign dummy.as_mut() to prev
    let mut prev = dummy.as_mut();
    // iterate on 0..m - 1 move prev by m nodes,
    for _ in 0..m - 1 {
        prev = prev.unwrap().next.as_mut();
    }
    // take the prev.next as mut and assign to current
    let mut current = prev.as_mut().unwrap().next.take();
    let mut next = current.as_mut().unwrap().next.take();
    // do the same with current and assign to next
    // iterate between m..n
    for _ in m..n {
        // do the above with next and assign it to temp
        let temp = next.as_mut().unwrap().next.take();
        // assign current to next's next (this is tricky)
        next.as_mut().unwrap().next = current;
        // assign next to current
        current = next;
        // assign temp to next
        next = temp;
    }
    // after iteration, assign next to prev.next.next
    prev.as_mut().unwrap().next.as_mut().unwrap().next = next;
    // after iteration, assign current to prev.next
    prev.as_mut().unwrap().next = current;
    // return dummy's next
    dummy.unwrap().next
}

pub fn make_list(in_list: Vec<i32>) -> Option<Box<Node>> {
    // this can be implement as from_vec fn
    // declare curr as None
    let mut curr = None;
    // use vector iter in reverse
    for vec in in_list.into_iter().rev() {
        // create new boxed nodes
        let mut node = Box::new(Node::new(vec));
        // assign curr to created node's next
        node.next = curr;
        // assign created node to curr
        curr = Some(node);
    }
    // return curr after iteration
    curr
}

pub fn make_tree(in_list: Vec<i32>) -> Option<Box<Binode>> {
    // traverses and attaches data to the tree nodes
    // check if vector is empty then return None
    if in_list.is_empty() {
        return None;
    }
    // assign a new boxed binode to curr
    let mut curr = Some(Box::new(Binode::new(in_list[0])));
    // assign a new vec wth curr to queue
    let mut queue = vec![curr.clone()];
    // declare mut idx to 1
    let mut idx = 1;
    // start while loop till idx < in_list.len
    while idx < in_list.len() {
        // in side the while loop, remove the 0th node from queue with if let some construct
        if let Some(mut node) = queue.remove(0) {
            // check if idx < in_list.len() and assign idx value to left node
            if idx < in_list.len() {
                node.left = Some(Box::new(Binode::new(in_list[idx])));
                queue.push(node.left.clone())
            }
            idx += 1;
            // push the created left node into queue
            if idx < in_list.len() {
                node.right = Some(Box::new(Binode::new(in_list[idx])));
                queue.push(node.right.clone())
            }
            // increment idx and do the same for right node and increment idx
            idx += 1;
        }
        // return curr
    }
    curr
}

pub fn depth_first_tt(root: &Option<Box<Binode>>, result: &mut Vec<i32>) {
    // use if let some on root
    if let Some(n) = root {
        // call self on n.left, along with result
        depth_first_tt(root, result);
        // push n.val into result
        result.push(n.val);
        // call self on n.right along with result
        depth_first_tt(root, result);
    }
}

use std::collections::VecDeque;

pub fn binary_first_tt(root: &Option<Box<Binode>>) -> Vec<i32> {
    // create result vector and queue VecDeque
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    // push root node into queue using if let constuct
    if let Some(node) = root {
        queue.push_back(node);
        // start a while let some constuct on queue.pop_front
        while let Some(curr) = queue.pop_front() {
            // push the value into result
            result.push(curr.val);
            // do if let constuct on left and right and push_back on queue
            if let Some(left) = &curr.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
            // after while construct return result
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
