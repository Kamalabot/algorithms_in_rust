use std::usize;

fn main() {
    println!("Sorting Practice session 01");
}

pub fn bubble_sort(in_list: Vec<i32>) -> Vec<i32> {
    // get length of the list

    // enumerate between 0 to len - 1
    // do nested enumerate between 0 to len - idx - 1
    // compare jdx + 1 < jdx
    // swap jdx + 1 with jdx
    // return the list
    in_list
}
pub fn insertion_sort(in_list: Vec<i32>) -> Vec<i32> {
    // create to mutable local list
    // start enumeration from 2nd element of the list with idx=1
    // inside the loop assign it to key variable
    // make the jdx to be one step behind idx
    // enter while loop when jdx >=0 and val at jdx is less than key
    // assign current jdx value to jdx + 1 location
    // decrement jdx inside while loop
    // return list
    in_list
}

pub fn partition(in_list: &mut [i32]) -> usize {
    // performs the sorting function inside the quicksort
    // get len of the list
    // assign last element to be the pivot
    // start the ref idx with 0
    // enumerate from 0 to len - 1
    // compare value at jdx < pivot,
    // swap it with the ref idx value
    // increment idx
    // outside the loop idx will be 2nd to last elements
    // before last, swap it with last element
    // return the idx
    0
}

pub fn quick_sort(in_list: &mut [i32]) {
    // cannot use vector for quicksort
    // due to the operations we are performing
    // start by getting the len
    // return if len is 1 or less
    // find the pivot_idx using the above partition function
    // recursively call quick_sort on in_list, upto pivot_idx
    // and from pivot_idx + 1 to end of list (..)
}
pub fn merge(arr: &mut [i32], mid: usize) {
    // supporting function for merge_sort below
    // observe the argument is simply an array, and
    // the size is not mentioned in signature
    // create left and right mutable vectors
    // declare i, j and k to 0
    // use while loop i and j are less than lengths of
    // left and right lists
    // compare element as i and j inside left and right lists
    // if the left is less or equal to right then assign left to arr[k]
    // else assign right to arr[k]
    // enumerate using while i / j is less than length of left / right,
    // assign remaining elements to arr[k] by incrementing
}
pub fn merge_sort(in_list: &mut [i32]) {
    // get length of in_list
    // if less than or equal to 1, return
    // get the mid at half of len
    // recursively call merge_sort on left and right side of mid
    // call merge on in_list with mid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let in_list = vec![8, 5, 2, 3, 1];
        assert_eq!(bubble_sort(in_list), vec![1, 2, 3, 5, 8]);
    }
    #[test]
    fn insertion_test() {
        let in_list = vec![8, 5, 2, 3, 1];
        assert_eq!(insertion_sort(in_list), vec![1, 2, 3, 5, 8]);
    }
    #[test]
    fn merge_test() {
        let mut in_list = [8, 5, 2, 3, 1];
        merge_sort(&mut in_list);
        println!("inlist: {:?}", in_list);
        assert_eq!(in_list, [1, 2, 3, 5, 8]);
    }
    #[test]
    fn quick_test() {
        let mut in_list = [8, 5, 2, 3, 1];
        // the list is sorted internally
        quick_sort(&mut in_list);
        println!("inlist: {:?}", in_list);
        assert_eq!(in_list, [1, 2, 3, 5, 8]);
    }
}
