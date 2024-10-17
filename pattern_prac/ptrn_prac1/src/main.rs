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
    // declare left, right for both ends and iter variables
    let mut left: usize = 0;
    let mut right: usize = in_list.len() - 1;
    let mut iter = 0;

    while left < right {
        let sum = in_list[left] + in_list[right];
        println!("Sum on iteration {} is {}", iter, sum);
        // the below if takes care of returning
        // Some(value)
        if target == sum {
            return Some([right, left]);
        } // the match usage is requiring same return types
        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => (),
        }
        iter += 1;
    }
    None
}
pub fn slide_win(in_list: Vec<i32>, k: usize) -> i32 {
    let mut max_sum = in_list[..k].iter().sum();
    let mut curr = max_sum;
    for idx in k..in_list.len() {
        curr += in_list[idx] - in_list[idx - k];
        max_sum = max(curr, max_sum);
        println!("iteration max sum: {}", max_sum);
    }
    println!("Final Maxsum: {}", max_sum);
    max_sum
}

pub fn monot_stack(in_list: Vec<i32>) -> Vec<i32> {
    // assume that none of the elements have
    // greatest elements
    let mut result_vec = vec![-1; in_list.len()];
    // let stack: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::with_capacity(in_list.len());
    for idx in 0..in_list.len() {
        // While stack is not empty and current element is greater than the element
        // corresponding to the index at the top of the stack
        while !stack.is_empty() && in_list[idx] > in_list[*stack.last().unwrap()] {
            // Pop the index from the stack and update the result_vec for that index
            result_vec[stack.pop().unwrap()] = in_list[idx];
        }
        stack.push(idx);
    }
    println!("Final Stack: {:?}", result_vec);
    result_vec
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn n_largest(in_list: Vec<i32>, n: usize) -> Vec<i32> {
    // the logic below keeps the heap to required n-largest
    // rest are popped
    let mut min_heap = BinaryHeap::with_capacity(n);
    for num in in_list {
        // what happens below is unclear
        println!("What happens in reverse num: {:?}", Reverse(num));
        min_heap.push(Reverse(num));

        if min_heap.len() > n {
            min_heap.pop();
            // when the length exceeds pop
            // the smallest item
        }
    }
    let collected_min: Vec<i32> = min_heap
        .into_sorted_vec()
        .into_iter()
        .map(|x| x.0) // here we are looking at tuple elem 0
        .collect();
    println!("Collected Min Heap: {:?}", collected_min[0]);
    collected_min
}

pub fn overlap_intl(mut in_list: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // let test = vec![(15, 6), (7, 10)];
    in_list.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_interval: Vec<(i32, i32)> = Vec::new();

    let mut curr_interval = in_list[0];

    for intl in in_list.into_iter().skip(1) {
        if curr_interval.1 >= intl.0 {
            //get the max of the intervals
            curr_interval.1 = curr_interval.1.max(intl.1);
        } else {
            merged_interval.push(curr_interval);
            // the last intl is checked alreadb
            // and assigned to curr_interval
            curr_interval = intl;
        }
    }
    merged_interval.push(curr_interval);
    println!("Merged intervals are: {:?}", merged_interval);
    merged_interval
}

pub fn modified_search(in_list: Vec<i32>, target: i32) -> Option<usize> {
    if in_list.is_empty() {
        return None;
    }
    println!("Inlist is: {:?}", in_list);
    let mut left = 0;
    let mut right = in_list.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if target == in_list[mid] {
            return Some(mid);
        }
        if in_list[left] <= in_list[mid] {
            if in_list[left] <= target && target < in_list[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if in_list[mid] < target && target <= in_list[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    None
}

pub fn bin_search(in_list: Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = in_list.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if target == in_list[mid] {
            return Some(mid);
        }
        if target > in_list[mid] {
            left += mid;
        } else {
            right -= mid
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
