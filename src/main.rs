//! This file serves to show rust memory safety mechanisms.
//!
//! - RAII is enforced: There is no way you can put something on the heap without managing it.
//! - Move semantics by default: Ownership is usually moved
//! - The Borrow Checker ensures that no race conditions can occur
//! - Lifetimes are checked
//!
//! Examples are derived from [rust by example](https://doc.rust-lang.org/rust-by-example/scope.html)


/// Primitives are passed by value
fn primitives_by_value(x: u32) {
    println!("passed value is {}", x);
}

/// This function takes ownership of the heap allocated memory,
/// which means ownership is transferred to the function.
/// Since nothing is returned from the function,
/// the memory is freed at the end of the function.
fn consume_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

/// Rust moves ownership (even though this looks like 'by value')
fn consume_value(s: Point) {
    println!("Consumed complex value: x {} y {} z {}", s.x, s.y, s.z);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn mutably_borrow_point(point: &mut Point) {
    point.z += 1;
}

/// You can return references from functions.
/// However, if there is more than one input reference,
/// the return value must have the same lifetime as the input references.
/// Rust *could* infer the lifetimes, but it is cautious and does not.
fn find_max<'l>(a: &'l i32, b: &'l i32) -> &'l i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// You can return references from mutably borrowing functions.
/// However, if there is more than one input reference,
/// we need to specify the lifetime of the return value.
///
/// Here, we have to specify that the return value has the same lifetime as the input reference `a`.
fn only_mutate_a<'l>(a: &'l mut i32, b: &i32) -> &'l i32 {
    *a += *b;
    a
}

/// Demo the borrow checker with references
/// Demo derived from [rust by example (borrowing section)](https://doc.rust-lang.org/rust-by-example/scope/borrow/alias.html)
///
/// Nice [visualization](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html) of lifetimes
///
/// Borrowing Rules
/// - Data can be immutably borrowed any number of times.
/// - Data can be mutably borrowed only once at a time i.e. there is exactly this one mutable reference.
/// - After the mutable borrow goes out of scope, the immutable borrows can be used again.
///     - or another mutable borrow can be created.
///
/// It may take some getting used to that the "scope" of a reference ends with its last usage and not with actual scopes.
fn demo_references() {
    println!("===== Borrowing &References =====");
    let mut point = Point { x: 12, y: -4, z: 8 };

    let borrowed_point = &point;
    let other_borrowed_point = &point;

    // Can't mutably borrow at this point because the references are still in use in the println!() statement.
    //let mut mutable_borrowed_point = &mut point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, other_borrowed_point.y, point.z
    );

    // Now we can mutably borrow

    // ... in a function
    mutably_borrow_point(&mut point);

    // ... or in the same scope
    let mutable_borrowed_point = &mut point;
    mutable_borrowed_point.x += 2;

    // Can't have any borrow at this point because the references are still in use in the println!() statement.
    //let borrowed_point = &point;

    println!(
        "Altered Point has coordinates: ({}, {}, {})",
        mutable_borrowed_point.x, mutable_borrowed_point.y, mutable_borrowed_point.z
    );

    println!("\n")
}

/// Show how RAII (RAII is still a horrible name. What about "Resource Lifecycle Management"?) is enforced:
/// There is no way you can put something on the heap without managing it.
///
/// --> There is no such thing as `new` in combination with raw pointers in Rust.
///
/// Demo derived from [rust by example (raii section)](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
fn demo_raii_is_enforced() {
    println!("===== RAII is enforced =====");

    let _box_1 = Box::new(52_i32);

    {
        let _box_2 = Box::new(53_i32);

        // `_box_2` is popped off the stack here. The "destructor" cleans up the heap memory.
        // You'll notice: the behaviour is very similar to `unique_ptr` in C++
    }

    println!("\n")
}

fn demo_by_value_for_primitives() {
    println!("===== Passing by value =====");

    let x = 5_u32;
    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    primitives_by_value(x);
    primitives_by_value(y);

    println!("x is {}, and y is {}", x, y);

    println!("\n")
}

fn demo_moved_ownership() {
    println!("===== Moving Ownership =====");

    let simple_struct = Point { x: 1, y: 2, z: -4 };

    consume_value(simple_struct);

    // simple_struct is gone  -> below is not working
    //consume_value(simple_struct);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5_i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;

    // `a` has ended -> below is not working
    // println!("a contains: {}", a);

    consume_box(b);

    // `b` has ended -> below is not working
    //println!("b contains: {}", b);

    println!("\n")
}

fn demo_lifetime_annotations() {
    println!("''''Lifetime annotations''''");
    let x = 5_i32;
    let y = -58_i32;

    println!("Larger value is {}", find_max(&x, &y));

    let mut z = 45_i32;
    println!("Mutated value is {}", only_mutate_a(&mut z, &y));

    println!("\n")
}

fn main() {
    demo_raii_is_enforced();
    demo_by_value_for_primitives();
    demo_moved_ownership();
    demo_references();
    demo_lifetime_annotations();
}
