//! This file intended to show rust memory safety mechanisms.
//!
//! For more information, please refer to the following resources:
//!
//! - Examples are derived from [rust by example](https://doc.rust-lang.org/rust-by-example/scope.html)

fn demo_by_value_for_primitives() {
    let x = 5_u32;
    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    primitives_by_value(x);
    primitives_by_value(y);

    println!("x is {}, and y is {}", x, y);
}

/// primitives are passed by value
fn primitives_by_value(x: u32) {
    println!("passed value is {}", x);
}

/// This function takes ownership of the heap allocated memory
fn consume_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` is destroyed and the memory freed
}

/// Rust moves ownership (even though this looks like 'by value', which would be a copy)
fn consume_value(s: SimpleStruct) {
    println!("Consumed complex value: a {} b {}", s.a, s.b);
}

#[derive(Debug)]
struct SimpleStruct {
    a: i32,
    b: i32,
}

fn demo_moved_ownership() {
    let simple_struct = SimpleStruct { a: 1, b: 2 };

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
}

fn main() {
    demo_by_value_for_primitives();

    demo_moved_ownership();

}
