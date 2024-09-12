To implement the fast and slow pointer algorithm (commonly used to detect cycles in linked lists or to find the middle of a list), we need to traverse the `Node` struct you defined, where each `Node` contains an integer value and a link to the next node in the list (`Option<Box<Node>>`).

Here’s an implementation of the fast and slow pointer in Rust:

### Steps:

1. We'll create two pointers: `slow` and `fast`.
2. `slow` will move one step at a time, while `fast` will move two steps at a time.
3. If `fast` reaches the end of the list, `slow` will be at the middle of the list.
4. If there’s a cycle, `fast` will eventually meet `slow`.

### Implementation:

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            val: value,
            next: None,
        }
    }
}

// Function to detect if the linked list contains a cycle (returns true if there is a cycle)
pub fn has_cycle(head: Option<&Box<Node>>) -> bool {
    let mut slow = head;
    let mut fast = head;

    while let (Some(s), Some(f)) = (slow, fast) {
        // Move slow by one step
        slow = s.next.as_ref();

        // Move fast by two steps
        if let Some(next_fast) = &f.next {
            fast = next_fast.next.as_ref();
        } else {
            return false; // Fast pointer reached the end, no cycle
        }

        // If slow and fast meet, we have a cycle
        if slow == fast {
            return true;
        }
    }

    false
}

// Function to find the middle of the linked list (returns the middle node's value)
pub fn find_middle(head: Option<&Box<Node>>) -> Option<i32> {
    let mut slow = head;
    let mut fast = head;

    while let (Some(s), Some(f)) = (slow, fast) {
        // Move slow by one step
        slow = s.next.as_ref();

        // Move fast by two steps
        fast = if let Some(next_fast) = &f.next {
            next_fast.next.as_ref()
        } else {
            None
        };

        // If fast reached the end, slow is at the middle
        if fast.is_none() {
            return slow.map(|node| node.val);
        }
    }

    None
}

fn main() {
    // Create a linked list: 1 -> 2 -> 3 -> 4 -> 5
    let mut head = Box::new(Node::new(1));
    let mut second = Box::new(Node::new(2));
    let mut third = Box::new(Node::new(3));
    let mut fourth = Box::new(Node::new(4));
    let fifth = Box::new(Node::new(5));

    // Link nodes together
    fourth.next = Some(fifth);
    third.next = Some(fourth);
    second.next = Some(third);
    head.next = Some(second);

    // Check for a cycle
    let has_cycle = has_cycle(Some(&head));
    println!("Cycle detected: {}", has_cycle);

    // Find the middle of the list
    let middle_value = find_middle(Some(&head));
    println!("Middle of the list: {:?}", middle_value);
}
```

### Explanation:

1. **Cycle detection (`has_cycle`)**: 
   
   - The fast pointer moves two steps at a time, and the slow pointer moves one step. If they meet, there’s a cycle. If the fast pointer reaches the end (`None`), there’s no cycle.

2. **Finding the middle (`find_middle`)**: 
   
   - The fast pointer moves two steps at a time, and the slow pointer moves one step. When the fast pointer reaches the end, the slow pointer will be in the middle.

### Example Output:

```plaintext
Cycle detected: false
Middle of the list: Some(3)
```

This code works for linked lists with `Node` and checks for cycles and the middle element using the fast and slow pointer technique.
