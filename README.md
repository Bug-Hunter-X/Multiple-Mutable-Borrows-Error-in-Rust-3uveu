This repository demonstrates a common error in Rust: multiple mutable borrows. The `bug.rs` file contains code that attempts to create two mutable references to the same variable, which is forbidden by Rust's borrow checker.  The `bugSolution.rs` file offers a solution illustrating how to correctly handle mutable borrows in such situations.