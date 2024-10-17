fn main() {
    println!("Sorting Practice session 01");
}

pub fn bubble_sort(arg_list: Vec<i32>) -> Vec<i32> {
    let mut in_list = arg_list.clone();
    // get length of the list
    let n = in_list.len();
    // enumerate between 0 to len - 1
    for idx in 0..n - 1 {
        // do nested enumerate between 0 to len - idx - 1
        for jdx in 0..n - idx - 1 {
            // compare jdx + 1 < jdx
            if in_list[jdx + 1] < in_list[jdx] {
                in_list.swap(jdx + 1, jdx)
            }
        }
    }
    // swap jdx + 1 with jdx
    // return the list
    in_list
}
pub fn insertion_sort(in_list: Vec<i32>) -> Vec<i32> {
    // create to mutable local list
    let mut loc_list = in_list.clone();
    let n = loc_list.len();
    // start enumeration from 2nd element of the list with idx=1
    for idx in 1..n {
        // inside the loop assign it to key variable
        let key = loc_list[idx];
        // make the jdx to be one step behind idx
        let mut jdx = idx as isize - 1;
        // enter while loop when jdx >=0 and key is less than val at jdx
        while jdx >= 0 && key < loc_list[jdx as usize] {
            // assign current jdx value to jdx + 1 location
            loc_list[jdx as usize + 1] = loc_list[jdx as usize];
            // decrement jdx inside while loop
            jdx -= 1;
        }
        // after the while loop, updt last elem with key
        loc_list[(jdx + 1) as usize] = key
    }
    // return list
    loc_list
}

pub fn partition(in_list: &mut [i32]) -> usize {
    // performs the sorting function inside the quicksort
    // get len of the list
    let n = in_list.len();
    // assign last element to be the pivot
    let pivot = in_list[n - 1];
    // start the ref idx with 0
    let mut idx = 0;
    // enumerate from 0 to len - 1
    for jdx in 0..n - 1 {
        // compare value at jdx < pivot,
        if in_list[jdx] < pivot {
            // swap it with the ref idx value
            in_list.swap(jdx, idx);
            // increment idx
            idx += 1;
        }
    }
    in_list.swap(idx, n - 1);
    // outside the loop idx will be 2nd to last elements
    // before last, swap it with last element
    // return the idx
    idx
}

pub fn quick_sort(in_list: &mut [i32]) {
    // cannot use vector for quicksort
    // due to the operations we are performing
    // start by getting the len
    let n = in_list.len();
    // return if len is 1 or less
    if n <= 1 {
        return;
    }
    // find the pivot_idx using the above partition function
    let pivot_idx = partition(in_list);
    // recursively call quick_sort on in_list, upto pivot_idx
    quick_sort(&mut in_list[0..pivot_idx]);
    // and from pivot_idx + 1 to end of list (..)
    quick_sort(&mut in_list[pivot_idx + 1..])
}
pub fn merge(arr: &mut [i32], mid: usize) {
    // supporting function for merge_sort below
    // observe the argument is simply an array, and
    // the size is not mentioned in signature
    // create left and right mutable vectors
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    // declare i, j and k to 0
    let mut idx = 0;
    let mut jdx = 0;
    let mut kdx = 0;
    // use while loop i and j are less than lengths of
    // left and right lists
    while idx < left.len() && jdx < right.len() {
        // compare element as i and j inside left and right lists
        if left[idx] <= right[jdx] {
            // if the left is less or equal to right then assign left to arr[k]
            arr[kdx] = left[idx];
            kdx += 1;
            idx += 1;
        // else assign right to arr[k]
        } else {
            arr[kdx] = right[jdx];
            kdx += 1;
            jdx += 1;
        }
    }
    // enumerate using while i / j is less than length of left / right,
    while idx < left.len() {
        arr[kdx] = left[idx];
        kdx += 1;
        idx += 1;
    }
    // assign remaining elements to arr[k] by incrementing
    while jdx < right.len() {
        arr[kdx] = right[jdx];
        kdx += 1;
        jdx += 1;
    }
}
pub fn merge_sort(in_list: &mut [i32]) {
    // get length of in_list
    let ln = in_list.len();
    // if less than or equal to 1, return
    if ln <= 1 {
        return;
    }
    // get the mid at half of len
    let mid = ln / 2;
    // recursively call merge_sort on left and right side of mid
    merge_sort(&mut in_list[0..mid]);
    merge_sort(&mut in_list[mid..]);
    // call merge on in_list with mid
    merge(in_list, mid);
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
