---
In place full linked list reversal
---



Here’s the implementation for reversing a linked list in place, with detailed comments:

```rust
impl Node {
    // Function to reverse the linked list in place
    pub fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
        let mut prev = None; // Initialize previous node as None
        let mut current = head; // Start with the head node

        // Loop through the list until current becomes None (end of the list)
        while let Some(mut curr_node) = current {
            let next = curr_node.next.take(); // Temporarily store the next node, and break the link
            curr_node.next = prev; // Reverse the link: current node's next now points to the previous node
            prev = Some(curr_node); // Move prev to the current node
            current = next; // Move current to the next node (previously stored)
        }

        // At the end of the loop, prev will be the new head of the reversed list
        prev
    }
}
```

### Comments:

- **`prev = None`**: Initializes the previous node as `None`, which will become the tail of the reversed list.
- **`current = head`**: The current node starts from the head of the original list.
- **`while let Some(mut curr_node)`**: Iterates through each node, unwrapping it from the `Option`.
- **`let next = curr_node.next.take()`**: Temporarily stores the next node and breaks the existing link.
- **`curr_node.next = prev`**: Points the current node's `next` to the previous node, reversing the link.
- **`prev = Some(curr_node)`**: Moves `prev` to the current node to continue reversing the next node.
- **`current = next`**: Advances the `current` pointer to the next node in the original list.
- **`prev` at the end**: This will hold the new head of the reversed list after the loop finishes.

Here’s the implementation for reversing part of a linked list (from position `m` to `n`) while keeping the rest of the list intact:

```rust
impl Node {
    // Function to reverse the list between positions m and n
    pub fn reverse_between(mut head: Option<Box<Node>>, m: usize, n: usize) -> Option<Box<Node>> {
        if m == n {
            return head; // No need to reverse if m and n are the same
        }

        let mut dummy = Some(Box::new(Node { val: 0, next: head })); // Dummy node to handle edge cases
        let mut prev = dummy.as_mut(); // Initialize previous node pointer to dummy

        // Step 1: Traverse to the (m-1)th node
        for _ in 0..m - 1 {
            prev = prev.unwrap().next.as_mut(); // Move prev to the (m-1)th node
        }

        // Now prev points to the (m-1)th node, and we'll start reversing from mth node
        let mut current = prev.as_mut().unwrap().next.take(); // The mth node
        let mut next = current.as_mut().unwrap().next.take(); // The (m+1)th node

        // Step 2: Reverse the sublist between m and n
        for _ in m..n {
            let temp = next.as_mut().unwrap().next.take(); // Temporarily store the (m+2)th node
            next.as_mut().unwrap().next = current; // Reverse the link: (m+1)th node now points to mth node
            current = next; // Move current to the next node (m+1 becomes m)
            next = temp; // Move next to the next node to be processed
        }

        // Step 3: Connect the reversed sublist back to the original list
        prev.as_mut().unwrap().next.as_mut().unwrap().next = next; // Connect the mth node to the (n+1)th node
        prev.as_mut().unwrap().next = current; // Connect the (m-1)th node to the new head of the reversed sublist (nth node)

        dummy.unwrap().next // Return the new head of the list
    }
}
```

### Comments:

- **`dummy`**: A dummy node is used to simplify edge cases (like when `m` is 1, meaning you are reversing from the head).
- **`prev`**: Pointer initialized to the dummy node, then moved to point to the node just before the `m`th node.
- **`current`**: The `m`th node, which becomes the starting point for the reversal process.
- **`next`**: The node following `current` (i.e., the `(m+1)`th node).
- **Reversing loop**: For every iteration, the link between `current` and `next` is reversed, advancing both `current` and `next` until the sublist is reversed.
- **Reconnecting**: After the reversal, the `(m-1)`th node is connected to the new head of the reversed sublist, and the original `m`th node (now the tail) is connected to the `(n+1)`th node.

This will reverse the part of the list between positions `m` and `n`, keeping the rest of the list unchanged.
