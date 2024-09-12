Here's an implementation of merging overlapping intervals in Rust, with detailed comments explaining the logic:

```rust
// Function to merge overlapping intervals
pub fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // If there are no intervals or just one, return them as is
    if intervals.is_empty() {
        return intervals;
    }

    // Sort intervals based on the starting value
    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    // Create a result vector to store merged intervals
    let mut merged_intervals: Vec<(i32, i32)> = Vec::new();

    // Initialize the first interval as the starting point
    let mut current_interval = intervals[0];

    // Iterate through the intervals and merge if overlapping
    for interval in intervals.into_iter().skip(1) {
        // Check if the current interval overlaps with the next one
        if current_interval.1 >= interval.0 {
            // If overlapping, merge the intervals by taking the max of their end values
            current_interval.1 = current_interval.1.max(interval.1);
        } else {
            // If not overlapping, push the current interval to the result and update it
            merged_intervals.push(current_interval);
            current_interval = interval;
        }
    }

    // Push the last interval
    merged_intervals.push(current_interval);

    merged_intervals
}

// Call Example:
// let intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
// let result = merge_intervals(intervals);
// Expected Output: [(1, 6), (8, 10), (15, 18)]
```

### Explanation:

- **Sorting**: The intervals are sorted by their start times. This helps in merging the intervals easily since overlapping intervals will appear consecutively.
- **Merging Logic**: 
  - If two intervals overlap (i.e., the start of the next interval is less than or equal to the end of the current interval), we merge them by extending the end of the current interval to the maximum end of the two intervals.
  - If they don't overlap, we push the current interval to the result and move on to the next.
- **Efficiency**: Sorting the intervals takes \(O(n \log n)\), and iterating through them takes \(O(n)\), making the overall time complexity \(O(n \log n)\).

This method ensures all overlapping intervals are merged into one, and non-overlapping intervals are preserved as is.
