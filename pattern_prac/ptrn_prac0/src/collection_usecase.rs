use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub fn simple_vec(nums: Vec<i32>) -> i32 {
    // getting the sum of squares from a list
    nums.into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum()
}

pub fn vecdeque() -> VecDeque<i32> {
    let mut vdq = VecDeque::new();
    vdq.push_back(1);
    vdq.push_back(2);
    vdq.push_front(3);
    println!("Print the vdq: {:?}", vdq);
    vdq
}

pub fn hmap() {
    let mut map = HashMap::new();
    map.insert("table", 3);
    map.insert("stands", 2);
    map.insert("tv", 1);
    println!("Keys {:?}", map.keys());
    println!("values {:?}", map.values());
    println!("What else?");

    let sentence = "This will make this example very this friendly";
    // counting and storing the words
    let split_sent = sentence.split_whitespace();
    let mut word_freq = HashMap::new();

    for word in split_sent {
        // super one line for checking the key
        // exists and then push it into the map
        *word_freq.entry(word).or_insert(0) += 1;
    }
    println!("Printing the freq_map: {:?}", word_freq);
}

pub fn hset() {
    let arr1 = [1, 2, 3, 5, 6];
    let arr2 = [2, 1, 7, 8, 16];
    let set1: HashSet<_> = arr1.into_iter().collect();
    let set2: HashSet<_> = arr2.into_iter().collect();

    let intset: HashSet<_> = set2.intersection(&set1).copied().collect();
    println!("The variables are {:?}", intset);
    let unionset: HashSet<_> = set2.union(&set1).copied().collect();
    println!("The union collection is: {:?}", unionset);
}

pub fn revllist(mut llist: LinkedList<i32>) -> LinkedList<i32> {
    // this is not inplace reversing, as a new
    // list is being created
    let mut reversed = LinkedList::new();
    while let Some(val) = llist.pop_back() {
        reversed.push_back(val);
    }
    reversed
}

pub fn binheap() -> Vec<i32> {
    let heap = BinaryHeap::from([5, 6, 8, 9, 0, 3, 4, 11, 86]);
    heap.into_iter().take(3).collect()
}

pub fn btreemap_example() -> BTreeMap<&'static str, i32> {
    let mut grades = BTreeMap::new();
    grades.insert("Alice", 85);
    grades.insert("Bob", 90);
    grades.insert("Charlie", 78);
    grades.insert("Dave", 92);
    // Grades are automatically sorted by name
    grades
}

pub fn btreeset_example(nums: Vec<i32>) -> BTreeSet<i32> {
    let mut set = BTreeSet::new();
    for num in nums {
        set.insert(num);
    }
    // Unique elements are automatically sorted
    set
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
