pub fn bubble_sort(in_list: Vec<i32>) -> Vec<i32> {
    in_list
}
pub fn insertion_sort(in_list: Vec<i32>) -> Vec<i32> {
    in_list
}
pub fn quick_sort(in_list: Vec<i32>) -> Vec<i32> {
    in_list
}
pub fn merge_sort(in_list: Vec<i32>) -> Vec<i32> {
    in_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(bubble_sort(in_list), vec![2, 3, 5, 6, 7, 9, 10, 12]);
    }
    #[test]
    fn insertion_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(insertion_sort(in_list), vec![2, 3, 5, 6, 7, 9, 10, 12]);
    }
    #[test]
    fn merge_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(merge_sort(in_list), vec![2, 3, 5, 6, 7, 9, 10, 12]);
    }
    #[test]
    fn quick_test() {
        let in_list = vec![5, 7, 6, 2, 3, 9, 10, 12];
        assert_eq!(quick_sort(in_list), vec![2, 3, 5, 6, 7, 9, 10, 12]);
    }
}
