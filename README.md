# Rust Safety Features Demoed

Sample code to demo some of Rust's safety features

- RAII is enforced: There is no way you can put something on the heap without managing it.
- Move semantics by default: Ownership is usually moved
- The Borrow Checker ensures that no race conditions can occur
- Lifetimes are checked
