// mod collection_usecase;

fn main() {
    println!("pattern practice session 1");
}

pub fn prefix_sum(in_list: Vec<i32>, i: usize, j: usize) -> i32 {
    // create prefix sum vector fill with 0
    let mut p = vec![0; in_list.len()];
    // assing val at 0 idx to p[0]
    p[0] = in_list[0];
    // enumerate from 1 to full_length of list
    for idx in 1..in_list.len() {
        // take prefix value at idx - 1, add it to curr value at in_list idx
        p[idx] = p[idx - 1] + in_list[idx];
        // assign it as prefix value at idx
    }
    // subtract the prefix value at j & i - 1 and return
    p[i] - p[j - 1]
}
use std::cmp::{max, Ordering};

pub fn two_pointer(in_list: Vec<i32>, target: i32) -> Option<[usize; 2]> {
    // declare left, right for two pointer & iter variable
    let mut left = 0;
    let mut right = in_list.len();
    while left < right {
        //  get the sum of value as left and right
        let sum = in_list[left] + in_list[right];
        //      Check if the sum is equal to target
        if sum == target {
            //      matches then return left and right
            return Some([left, right]);
        } else {
            //      else run a match on sum.cmp(&target)
            match sum.cmp(&target) {
                //          if sum is less then move left by 1
                Ordering::Less => left += 1,
                //          is greater then move right one step back
                Ordering::Greater => right -= 1,
                //          if equal then unit
                Ordering::Equal => (),
            }
        }
        //  increase iter by 1
    }
    // not finding then return None
    None
}
pub fn slide_win(in_list: Vec<i32>, k: usize) -> i32 {
    // get the sum of elements upto k
    let mut max_sum: i32 = in_list.iter().take(k).sum();
    // make that sum as curr
    let mut curr = max_sum;
    // enuerate from k to end of list
    for idx in k..in_list.len() {
        // add the val at idx and subtract val at idx - k to curr
        curr += in_list[idx] - in_list[idx - k];
        // assign max of max_sum & curr to max_sum
        max_sum = curr.max(max_sum);
    }
    // return max_sum
    max_sum
}

pub fn monot_stack(in_list: Vec<i32>) -> Vec<i32> {
    // assume that none of the elements have
    // greatest elements, all -1 in result vector
    let mut mono = vec![-1; in_list.len()];
    // create stack with with_capacity equal to in_list length
    let mut stack = Vec::with_capacity(in_list.len());
    // enumerate 0 to length of list
    for idx in 0..in_list.len() {
        // While stack is not empty and current element is greater than the element
        while !stack.is_empty() && in_list[idx] > in_list[*stack.last().unwrap()] {
            // corresponding to the index at the top of the stack (*stack.last().unwrap())
            mono[stack.pop().unwrap()] = in_list[idx];
            // Pop the index from the stack and update the result_vec at the returned index
        }
        // push idx into stack, outside while loop
        stack.push(idx);
    }
    mono
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn n_largest(in_list: Vec<i32>, n: usize) -> Vec<i32> {
    // the logic below keeps the heap to required n-largest
    // rest are popped
    // create a BinaryHeap with capacity of n
    let mut binheap = BinaryHeap::with_capacity(n);
    // enumerate the elem of in_list and push into heap with reverse
    for idx in in_list {
        // what happens below is unclear
        println!("{:?}", Reverse(idx));
        binheap.push(Reverse(idx));
        // when the length exceeds pop
        if binheap.len() > n {
            binheap.pop();
        }
    }
    // the smallest item
    let ret_vec: Vec<i32> = binheap.into_sorted_vec().into_iter().map(|x| x.0).collect();
    // collect the remaining n elem after extracting the 1st elem
    // of the tuple, as i32 vectors
    // return the i32 vecto
    ret_vec
}

pub fn overlap_intl(mut in_list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // let test = vec![(15, 6), (7, 10)];
    // sort the intervals using a.0.cmp(b.0)
    in_list.sort_by(|a, b| a.0.cmp(&b.0));
    // create a empty merged_interval vector
    let mut merged = Vec::new();
    // assign curr to first interval
    let mut curr = in_list[0];
    // enumerate intervals in the list by skipping 1st
    for intl in in_list.into_iter().skip(1) {
        // check  if curr.1 >= intl_elm.0
        if curr.1 >= intl.0 {
            //get the max of the intervals
            curr.1 = intl.1.max(curr.1);
        } else {
            merged.push(curr);
            curr = intl;
        }
    }
    // the last intl is checked alreadb
    merged.push(curr);
    // and assigned to curr_interval
    // push the last interval
    merged
}

pub fn modified_search(in_list: Vec<i32>, target: i32) -> Option<usize> {
    // if in_list is empty, then return None
    if in_list.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = in_list.len() - 1;
    // assign left and right, right to len() - 1
    while left <= right {
        // get mid by left + (right - left) / 2
        let mid = left + (right - left) / 2;
        // check if target is at mid then return mid
        if target == in_list[mid] {
            return Some(mid);
        }
        // check if val @ left <= val @ mid
        if in_list[left] <= in_list[mid] {
            // if yes, then check if target b/w left & mid
            if in_list[left] <= target && target < in_list[mid] {
                // if yes then make right to move to mid - 1
                right = mid - 1;
            // else left to move to mid + 1
            } else {
                left = mid + 1;
            }
        } else {
            // if val @ left > val @ mid
            if in_list[mid] < target && target <= in_list[right] {
                // check if target is b/w mid and right
                left = mid + 1
            //  if yes then move left to mid + 1
            } else {
                //  else move right to mid - 1
                right = mid - 1;
            }
        }
    }

    None
}

pub fn bin_search(in_list: Vec<i32>, target: i32) -> Option<usize> {
    // assign left and right,
    let mut left = 0;
    let mut right = in_list.len() - 1;
    // while left <= right
    while left <= right {
        // get the mid value, check if target at mid and return
        let mid = left + (right - left) / 2;
        // if not, check if target is greater than mid,
        if target == in_list[mid] {
            return Some(mid);
        }
        if target > in_list[mid] {
            // yes then move left to  left +  mid
            left += mid;
        // no then move right to right - mid
        } else {
            right -= mid;
        }
    }
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
