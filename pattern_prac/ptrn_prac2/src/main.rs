mod collection_usecase;

fn main() {
    println!("pattern practice session 1");
}

pub fn prefix_sum(in_list: Vec<i32>, i: usize, j: usize) -> i32 {
    // create prefix sum vector fill with 0
    // assing val at 0 idx to p[0]
    // enumerate from 1 to full_length of list
    // take prefix value at idx - 1, add it to curr value at in_list idx
    // assign it as prefix value at idx
    // subtract the prefix value at j & i - 1 and return
}
use std::cmp::{max, Ordering};

pub fn two_pointer(in_list: Vec<i32>, target: i32) -> Option<[usize; 2]> {
    // declare left, right for two pointer & iter variable
    // while left < right
    //  get the sum of value as left and right
    //      Check if the sum is equal to target
    //      matches then return left and right
    //      else run a match on sum.cmp(&target)
    //          if sum is less then move left by 1
    //          is greater then move right one step back
    //          if equal then unit
    //  increase iter by 1
    // not finding then return None
    None
}
pub fn slide_win(in_list: Vec<i32>, k: usize) -> i32 {
    // get the sum of elements upto k
    // make that sum as curr
    // enuerate from k to end of list
    // add the val at idx and subtract val at idx - k to curr
    // assign max of max_sum & curr to max_sum
    // return max_sum
    0
}

pub fn monot_stack(in_list: Vec<i32>) -> Vec<i32> {
    // assume that none of the elements have
    // greatest elements, all -1 in result vector
    // create stack with with_capacity equal to in_list length
    // enumerate 0 to length of list
    // While stack is not empty and current element is greater than the element
    // corresponding to the index at the top of the stack (*stack.last().unwrap())
    // Pop the index from the stack and update the result_vec at the returned index
    // push idx into stack, outside while loop
    in_list
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn n_largest(in_list: Vec<i32>, n: usize) -> Vec<i32> {
    // the logic below keeps the heap to required n-largest
    // rest are popped
    // create a BinaryHeap with capacity of n
    // enumerate the elem of in_list and push into heap with reverse
    // what happens below is unclear
    // when the length exceeds pop
    // the smallest item
    // collect the remaining n elem after extracting the 1st elem
    // of the tuple, as i32 vectors
    // return the i32 vecto
    in_list
}

pub fn overlap_intl(mut in_list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // let test = vec![(15, 6), (7, 10)];
    // sort the intervals using a.0.cmp(b.0)
    // create a empty merged_interval vector
    // assign curr to first interval
    // enumerate intervals in the list by skipping 1st
    // check  if curr.1 >= intl_elm.0
    //get the max of the intervals
    // the last intl is checked alreadb
    // and assigned to curr_interval
    // push the last interval
    in_list
}

pub fn modified_search(in_list: Vec<i32>, target: i32) -> Option<usize> {
    // if in_list is empty, then return None
    // assign left and right, right to len() - 1
    // while left < right
    // get mid by left + (right - left) / 2
    // check if target is at mid then return mid
    // check if val @ left <= val @ mid
    // if yes, then check if target b/w left & mid
    // if yes then make right to move to mid - 1
    // else left to move to mid + 1

    // if val @ left > val @ mid
    // check if target is b/w mid and right
    //  if yes then move left to mid + 1
    //  else move right to mid - 1
    None
}

pub fn bin_search(in_list: Vec<i32>, target: i32) -> Option<usize> {
    // assign left and right,
    // while left <= right
    // get the mid value, check if target at mid and return
    // if not, check if target is greater than mid,
    // yes then move left to  left +  mid
    // no then move right to right - mid

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_sum_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(prefix_sum(in_list, 1, 3), 15);
    }

    #[test]
    fn slide_win_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(slide_win(in_list, 3), 31);
    }

    #[test]
    fn two_pointer_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(two_pointer(in_list, 22), Some([7, 6]));
    }

    #[test]
    fn monot_stack_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(monot_stack(in_list), [7, 9, 9, 3, 9, 10, 12, -1]);
    }

    #[test]
    fn n_largest_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(n_largest(in_list, 2), vec![12, 10]);
    }
    #[test]
    fn mod_search_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(modified_search(in_list, 5), Some(0));
        // the above func will consume in_list
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(modified_search(in_list, 25), None);
    }
    #[test]
    fn bin_search_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(bin_search(in_list, 2), Some(3));
    }
    #[test]
    fn overlap_intl_test() {
        let in_list = vec![(5, 7), (2, 6), (3, 9), (10, 12)];
        assert_eq!(overlap_intl(in_list), vec![(2, 9), (10, 12)]);
    }
}
