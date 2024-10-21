#![allow(unused_variables)]
use std::fmt::Write;
//Create a Node struct with val and next
//impl a new fn
//derive PartialEq, Eq with Debug

// Implement Binode struct with val, left & right
// PartialEq, Eq is required

// Write a function that traverses stack and prints
pub fn stack_traverse(root: Option<Box<Node>>) -> String {
    // The type that we are choosing to send in
    // will decide how they behave inside the function
    // declare empty output string, and make root node as curr
    // use while let Some constuct
    // write! node.val to output and move curr to next
    "output".to_owned()
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
    None
}

pub fn inplace_reversal(root: Option<Box<Node>>) -> Option<Box<Node>> {
    // make prev as none and current as root
    // use while let some construct on current
    // take the next node, using curr_node.next
    // make curr_node.next as prev
    // assign Some(curr_node) to prev
    // assign next to current
}

pub fn reverse_between(mut head: Option<Box<Node>>, m: usize, n: usize) -> Option<Box<Node>> {
    // if m is n then return head
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
}

pub fn make_list(in_list: Vec<i32>) -> Option<Box<Node>> {
    // this can be implement as from_vec fn
    // declare curr as None
    // use vector iter in reverse
    // create new boxed nodes
    // assign curr to created node's next
    // assign created node to curr
    // return curr after iteration
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
}

pub fn depth_first_tt(root: &Option<Box<Binode>>, result: &mut Vec<i32>) {
    // use if let some on root
    // call self on n.left, along with result
    // push n.val into result
    // call self on n.right along with result
}

use std::collections::VecDeque;

pub fn binary_first_tt(root: &Option<Box<Binode>>) -> Vec<i32> {
    // create result vector and queue VecDeque
    // push root node into queue using if let constuct
    // start a while let some constuct on queue.pop_front
    // push the value into result
    // do if let constuct on left and right and push_back on queue
    // after while construct return result
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
