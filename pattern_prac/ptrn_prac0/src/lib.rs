pub fn prefix_sum(in_list: Vec<i32>, i: usize, j: usize) -> i32 {
    let mut p = vec![0; in_list.len()]; // this must fill with 0
                                        // let p = (0..in_list.len()).collect() is anothe option
                                        // println!("P is {:?}", p);
                                        // iterate over the elm with idx
    p[0] = in_list[0];
    for idx in 1..in_list.len() {
        p[idx] = p[idx - 1] + in_list[idx];
    }
    // println!("prefix summed {:?}", p);
    p[j] - p[i - 1]
}
use std::cmp::{max, Ordering};

pub fn two_pointer(in_list: Vec<i32>, target: i32) -> Option<[usize; 2]> {
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

pub fn fast_slow(in_list: Vec<i32>) -> bool {
    true
}

pub fn monot_stack(in_list: Vec<i32>) -> Vec<i32> {
    in_list
}

pub fn n_largest(in_list: Vec<i32>) -> i32 {
    6
}

pub fn overlap_intl(in_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vec![vec![5, 6], vec![7, 10]]
}

pub fn modified_search(in_list: Vec<i32>, target: i32) -> i32 {
    3
}

pub fn bin_search(in_list: Vec<i32>, target: i32) -> i32 {
    3
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
        assert_eq!(monot_stack(in_list), [4, 2, 4, -1, -1, -1]);
    }

    #[test]
    fn n_largest_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(n_largest(in_list), 6);
    }
    #[test]
    fn mod_search_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(modified_search(in_list, 5), 0);
    }
    #[test]
    fn bin_search_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(bin_search(in_list, 2), 3);
    }
    #[test]
    fn overlap_intl_test() {
        let in_list = vec![vec![5, 7], vec![2, 6], vec![3, 9], vec![10, 12]];
        assert_eq!(overlap_intl(in_list), vec![vec![2, 9], vec![10, 12]]);
    }
}
