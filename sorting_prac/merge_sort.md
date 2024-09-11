Here’s the implementation of **Merge Sort** using recursion in Rust:

### Merge Sort Implementation in Rust

```rust
fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();

    // Base case: arrays of length 1 are already sorted
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // Split the array into two halves
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);

    // Merge the sorted halves
    merge(arr, mid);
}

// Helper function to merge two sorted subarrays
fn merge(arr: &mut [i32], mid: usize) {
    let mut left = arr[0..mid].to_vec(); // Copy left half
    let mut right = arr[mid..].to_vec(); // Copy right half

    let mut i = 0; // Iterator for left
    let mut j = 0; // Iterator for right
    let mut k = 0; // Iterator for arr

    // Merge the two halves
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

    // Copy the remaining elements of left, if any
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    // Copy the remaining elements of right, if any
    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
```

### Explanation:

1. **`merge_sort` function**:
   
   - This function divides the array into two halves and recursively calls itself on each half until the base case is reached (a single element).
   - The array is then merged back together in sorted order using the `merge` function.

2. **`merge` function**:
   
   - This function merges two sorted subarrays (left and right halves) into one sorted array.
   - Two indices (`i` and `j`) track positions in the left and right halves, and `k` tracks the position in the original array (`arr`).
   - The merging continues until all elements from both subarrays are placed back into `arr`.

### Example Usage:

```rust
fn main() {
    let mut arr = [38, 27, 43, 3, 9, 82, 10];
    println!("Unsorted array: {:?}", arr);

    merge_sort(&mut arr);

    println!("Sorted array: {:?}", arr);
}
```

### Output:

```plaintext
Unsorted array: [38, 27, 43, 3, 9, 82, 10]
Sorted array: [3, 9, 10, 27, 38, 43, 82]
```

### Key Points:

- **Divide and Conquer**: Merge sort recursively splits the array into two halves, sorts each half, and then merges the two halves.
- **Stable Sorting Algorithm**: Merge sort is a stable sort (it preserves the relative order of equal elements).
- **Time Complexity**: O(n log n), where `n` is the number of elements in the array.j
