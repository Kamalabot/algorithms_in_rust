use std::usize;

pub fn bubble_sort(in_list: Vec<i32>) -> Vec<i32> {
    let mut loc_list = in_list.clone();
    // the function takes in a mutable list
    // enumerate on the list twice
    let n = loc_list.len();
    for idx in 0..n - 1 {
        for jdx in 0..n - idx - 1 {
            if loc_list[jdx + 1] < loc_list[jdx] {
                loc_list.swap(jdx + 1, jdx)
            }
        }
    }
    // println!("Sorted List: {:?}", loc_list);
    loc_list
}
pub fn insertion_sort(in_list: Vec<i32>) -> Vec<i32> {
    let mut loc_list = in_list;
    // start 2nd element of the list idx=1
    for idx in 1..loc_list.len() {
        let key = loc_list[idx];
        // jdx will track from 1st element
        let mut jdx = idx as isize - 1;

        while jdx >= 0 && key < loc_list[jdx as usize] {
            loc_list[jdx as usize + 1] = loc_list[jdx as usize];
            // this will throw error as the
            // value wont become negative
            jdx -= 1
        }
        // even here you to keep isize jdx and then cast it
        loc_list[(jdx + 1) as usize] = key;
    }
    loc_list
}

pub fn partition(in_list: &mut [i32]) -> usize {
    let len = in_list.len();
    let pivot = in_list[len - 1];
    let mut idx = 0;
    // which ever list comes in, is getting
    // sorted below using the pivot
    for jdx in 0..len - 1 {
        // check if the elm is less than pivot
        if in_list[jdx] < pivot {
            in_list.swap(idx, jdx);
            idx += 1; // swap & increment
        }
    }
    in_list.swap(idx, len - 1);
    idx
}

pub fn quick_sort(in_list: &mut [i32]) {
    // cannot use vector for quicksort
    // due to the operations we are performing
    let len = in_list.len();
    if len <= 1 {
        return;
    }
    let pivot_idx = partition(in_list);
    // the above happens every time below
    // recursive call is carried out
    quick_sort(&mut in_list[0..pivot_idx]);
    quick_sort(&mut in_list[pivot_idx + 1..]);
}
pub fn merge(arr: &mut [i32], mid: usize) {
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
pub fn merge_sort(in_list: &mut [i32]) {
    let len = in_list.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut in_list[0..mid]);
    merge_sort(&mut in_list[mid..]);

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
