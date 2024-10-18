use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub fn simple_vec(nums: Vec<i32>) -> i32 {
    // getting the sum of squares from a list
    nums.iter().map(|x| x * x).sum()
}

pub fn vecdeque() -> VecDeque<i32> {
    // create a vecdeque obj
    let mut vdq = VecDeque::from([1, 2, 7]);
    // push 1, 2 to back
    vdq.push_back(1);
    // push 3 to front
    vdq.push_front(5);
    // print and return obj
    println!("Vector Deque: {:?}", vdq);
    vdq
}

pub fn hmap() {
    // create new HashMap obj
    let mut hmap = HashMap::new();
    // insert count of 3 objects in the room you are in
    hmap.insert("practice", 3);
    hmap.insert("new_area", 1);
    hmap.insert("Threading", 3);
    // print the keys
    println!("{:?}", hmap.keys());
    // print the values
    println!("{:?}", hmap.values());
    // practice .entry and .or_insert(0) method with new HashMap
    *hmap.entry("new_area").or_insert(1) += 1;
    // print the new HashMap
}

pub fn hset() {
    // create two i32 arrays
    // let arr1: Vec<i32> = [10..25].iter().collect();
    // [a..b] purpose is different, used for indexing
    let a1 = [2, 5, 7, 9, 8, 3, 5];
    let b1 = [7, 6, 4, 2, 1, 3, 9];
    // use type annotation to collect the values into HashSet
    let hs1: HashSet<i32> = HashSet::from(a1);
    let hs2: HashSet<i32> = HashSet::from(b1);
    // use .intersection and .copied method
    let hs_inter: HashSet<i32> = hs1.intersection(&hs2).copied().collect();
    // use .union and .copied method
    let hs_union: HashSet<i32> = hs1.union(&hs2).copied().collect();
    // print the results
    println!("{:?}", hs_inter);
    println!("{:?}", hs_union);
}

pub fn revllist(mut llist: LinkedList<i32>) -> LinkedList<i32> {
    // Take a LinkedList obj from arguments
    // create a empty llist obj for reversed elements
    let mut rev_ll = LinkedList::new();
    // use while let on pop_back method to get value
    while let Some(val) = llist.pop_back() {
        rev_ll.push_back(val);
    }
    // use push_back method on new list to reverse the list
    // this is not inplace reversing, as a new
    // list is being created, and returned
    rev_ll
}

pub fn binheap() -> Vec<i32> {
    // create a binheap obj from a i32 array
    let btree = BinaryHeap::from([5, 7, 9, 2, 3, 8]);
    // use into_iter() and take 3 top elements into a i32 vector
    let take_3 = btree.into_iter().take(3).collect();
    // return that vector
    take_3
}

pub fn btreemap_example() -> BTreeMap<&'static str, i32> {
    // create a new BTreeMap obj
    let mut btreemap = BTreeMap::new();
    // insert marks of four fictious students
    btreemap.insert("mjo", 15);
    btreemap.insert("jlo", 75);
    btreemap.insert("Bluo", 35);
    btreemap.insert("Maco", 86);
    // return the btmap
    btreemap
}

pub fn btreeset_example(nums: Vec<i32>) -> BTreeSet<i32> {
    // take i32 vector
    let i32_vec = [5, 7, 9, 8, 6, 2];
    // create a new BTreeSet
    let mut btset = BTreeSet::from(i32_vec);
    // insert element of vector into BTreeSet
    btset.insert(8);
    // retrn set
    btset
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
}
