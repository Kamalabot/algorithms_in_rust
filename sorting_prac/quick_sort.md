Here’s the implementation of **Quicksort** using a recursive approach in Rust:

### Quicksort Implementation in Rust:

```rust
fn quicksort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]); // Recursively sort the left side
    quicksort(&mut arr[pivot_index + 1..]); // Recursively sort the right side
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1]; // Choose the last element as the pivot
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1); // Move the pivot to its correct position
    i
}
```

### Explanation:

1. **`quicksort` function:**
   
   - This function takes a mutable slice (`&mut [i32]`) as input, which is a subarray of the array you want to sort.
   - If the array has only one element or is empty (`len <= 1`), it's already sorted, so the function returns.
   - The `partition` function is used to place the pivot element in the correct position.
   - After partitioning, the array is split into two parts: one to the left of the pivot and one to the right. The `quicksort` function is then called recursively on both parts.

2. **`partition` function:**
   
   - The pivot is chosen as the last element in the array (`arr[len - 1]`).
   - The function then iterates over the array, swapping elements so that all elements less than the pivot are on the left, and all elements greater than the pivot are on the right.
   - Finally, the pivot is swapped into its correct position (`i`), and that position is returned.

### Example Usage:

```rust
fn main() {
    let mut arr = [10, 80, 30, 90, 40, 50, 70];
    println!("Unsorted array: {:?}", arr);

    quicksort(&mut arr);

    println!("Sorted array: {:?}", arr);
}
```

### Output:

```plaintext
Unsorted array: [10, 80, 30, 90, 40, 50, 70]
Sorted array: [10, 30, 40, 50, 70, 80, 90]
```

### Key Points:

- **Pivot Selection**: The pivot is chosen as the last element of the array (this could be modified to select a different pivot, such as the first element or a random element).
- **In-place Sorting**: The sorting is done in-place using swaps, so no additional memory is required other than the input array.
- **Recursive Calls**: The function calls itself recursively on the left and right subarrays created by partitioning around the pivot.
