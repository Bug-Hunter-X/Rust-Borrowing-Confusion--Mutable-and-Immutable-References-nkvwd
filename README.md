# Rust Borrowing Confusion: Mutable and Immutable References

This repository demonstrates a subtle point in Rust's borrowing system that can be confusing for beginners.  The example shows that a mutable reference (`&mut`) can coexist with an immutable reference (`&`) to the same data, as long as the immutable reference doesn't attempt to access the data while it is being modified by the mutable reference.

## The Bug

The `bug.rs` file contains code that initially might seem incorrect. A mutable reference `y` and an immutable reference `z` point to the same data. However, the code still allows modification via `y` without any compiler error.  This is because `z` only observes the data, it doesn't actively prevent modification.

## The Solution

The `bugSolution.rs` file shows the same basic code, but with comments explaining why the code is valid in Rust's borrowing system. Understanding that immutable references don't block mutations unless they are actively accessing data during the mutation is key.