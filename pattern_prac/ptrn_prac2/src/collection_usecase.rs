use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub fn simple_vec(nums: Vec<i32>) -> i32 {
    // getting the sum of squares from a list
}

pub fn vecdeque() -> VecDeque<i32> {
    // create a vecdeque obj
    // push 1, 2 to back
    // push 3 to front
    // print and return obj
}

pub fn hmap() {
    // create new HashMap obj
    // insert count of 3 objects in the room you are in
    // print the keys
    // print the values
    // practice .entry and .or_insert(0) method with new HashMap
    // print the new HashMap
}

pub fn hset() {
    // create two i32 arrays
    // use type annotation to collect the values into HashSet
    // use .intersection and .copied method
    // use .union and .copied method
    // print the results
}

pub fn revllist(mut llist: LinkedList<i32>) -> LinkedList<i32> {
    // Take a LinkedList obj made from i32 array
    // create a empty llist obj for reversed elements
    // use while let on pop_back method to get value
    // use push_back method on new list to reverse the list
    // this is not inplace reversing, as a new
    // list is being created, and returned
    llist
}

pub fn binheap() -> Vec<i32> {
    // create a binheap obj from a i32 array
    // use into_iter() and take 3 top elements into a i32 vector
    // return that vector
}

pub fn btreemap_example() -> BTreeMap<&'static str, i32> {
    // create a new BTreeMap obj
    // insert marks of four fictious students
    // return the btmap
}

pub fn btreeset_example(nums: Vec<i32>) -> BTreeSet<i32> {
    // take i32 vector
    // create a new BTreeSet
    // insert element of vector into BTreeSet
    // retrn set
}
#[cfg(test)]
mod tests {
    // obseve that the crate is taking from the
    // root, collection_usecase and then its functions
    use crate::collection_usecase::*;

    #[test]
    fn test_vec() {
        let invec = vec![6, 8, 7, 2, 3, 4, 5, 9, 1];
        let sum_out = simple_vec(invec);
        println!("Sum is: {:?}", sum_out);
        // assert!(true)
    }

    #[test]
    // #[ignore = "time"]
    fn test_vecdq() {
        let justdq = vecdeque();
        println!("Sum is: {:?}", justdq);
        // assert!(true)
    }
    #[test]
    // #[ignore = "time"]
    fn test_hashmap() {
        println!("Entering test_hashmap scope");
        hmap()
        // assert!(true)
    }
    #[test]
    // #[ignore = "time"]
    fn test_hashset() {
        println!("Entering test_hashset scope");
        hset()
        // assert!(true)
    }
    #[test]
    // #[ignore = "time"]
    fn test_linklist() {
        println!("Entering test_linklist scope");
        let newlist = LinkedList::from([1, 2, 3, 4, 5, 6, 7]);
        let revlist = revllist(newlist);
        println!("The updated list is {:?}", revlist);
        // assert!(true)
    }
    #[test]
    // #[ignore = "time"]
    fn test_bheap() {
        println!("Entering test_bheap scope");
        let heaped = binheap();
        println!("Getting the vector: {:?}", heaped);
        // assert!(true)
    }
    #[test]
    // #[ignore = "time"]
    fn test_btmap() {
        println!("Entering test_btmap scope");
        let btmaped = btreemap_example();
        println!("Getting the sorted btmap: {:?}", btmaped);
        // assert!(true)
    }
    #[test]
    // #[ignore = "time"]
    fn test_btmap() {
        println!("Entering test_btmap scope");
        // Call: let res = btreeset_example(vec![4, 2, 1, 3, 2, 5, 1]);
        let btmaped = btreemap_example();
        println!("Getting the sorted btmap: {:?}", btmaped);
        // assert!(true)
    }
}
