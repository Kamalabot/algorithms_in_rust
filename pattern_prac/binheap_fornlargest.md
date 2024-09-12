Rust does not have a direct equivalent to Python's `heapq` in the standard library. However, you can use the `BinaryHeap` from the `std::collections` module, which is a max-heap by default. To implement a min-heap or extract the n-largest elements, you can invert the order.

Here's a concise min-heap implementation using `BinaryHeap` to extract the n-largest elements:

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn n_largest(nums: Vec<i32>, n: usize) -> Vec<i32> {
    // Create a min-heap using Reverse to store the largest n elements
    let mut min_heap = BinaryHeap::with_capacity(n);

    for num in nums {
        // Push each element into the heap,
        // wrapping it with Reverse for a min-heap
        // why not use - sign
        min_heap.push(Reverse(num));

        // If heap size exceeds n, pop the smallest element (maintaining n largest elements)
        if min_heap.len() > n {
            min_heap.pop();
        }
    }

    // Collect the n largest elements, remove Reverse to get the original values
    min_heap.into_sorted_vec()
        .into_iter()
        .map(|x| x.0).collect()
}
```

### Explanation (in comments):

- **`BinaryHeap<Reverse<i32>>`**: Used to create a min-heap, as `BinaryHeap` is a max-heap by default.
- **`min_heap.push(Reverse(num))`**: Pushes each element wrapped in `Reverse` to maintain a min-heap.
- **`if min_heap.len() > n { min_heap.pop(); }`**: Ensures the heap only holds the n-largest elements.
- **`into_sorted_vec()`**: Returns the elements in ascending order.

This code extracts the n-largest elements from the list.
