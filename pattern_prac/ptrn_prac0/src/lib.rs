pub fn prefix_sum(in_list: Vec<i32>) -> i32 {
    0
}

pub fn two_pointer(in_list: Vec<i32>, target: i32) -> i32 {
    6
}
pub fn slide_win(in_list: Vec<i32>) -> i32 {
    5
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
        assert_eq!(prefix_sum(in_list), 10);
    }

    #[test]
    fn slide_win_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(slide_win(in_list), 10);
    }

    #[test]
    fn two_pointer_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(two_pointer(in_list, 3), 4);
    }

    #[test]
    fn fast_slow_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert!(fast_slow(in_list));
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
