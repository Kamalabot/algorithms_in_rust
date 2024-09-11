The issue arises because Rust expects all arms of a `match` expression to return the same type. In your code, the first `match` arm returns a `Some([right, left])`, which is an `Option<[usize; 2]>`, while the other arms (`left += 1` and `right -= 1`) don't return anything (i.e., they return `()`).

In Rust, you cannot mix return types in different `match` arms. Here's how you can fix this by handling the `return` separately from the `match`:

### Corrected Code:

```rust
if target == sum {
    return Some([right, left]);
}

match target.cmp(&sum) {
    std::cmp::Ordering::Less => left += 1,
    std::cmp::Ordering::Greater => right -= 1,
    std::cmp::Ordering::Equal => (), // This won't happen as the equality check is handled before
}
```

### Explanation:

1. **Separate `if` for equality**: The `if target == sum` condition is checked before the `match`, which avoids having to return from the `match`.
2. **`match` handles only comparisons (`Less` and `Greater`)**: Since the equality check is already handled by the `if` statement, the `match` only needs to adjust `left` and `right` based on whether `target` is less than or greater than `sum`.

This should resolve the issue with mismatched types.
