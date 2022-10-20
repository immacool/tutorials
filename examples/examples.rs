
// 1. Variables and mutability
// 2. Data types
// 3. Functions
// 4. Comments
// 5. Control flow
// 6. Loops
// 7. Ownership
// 8. References and borrowing
// 9. Structs
// 10. Enums and pattern matching
// 11. Modules
// 12. Object Oriented programming
// 13. Error Handling
// 14. Generic Types


// 1) ================================= Variables and Mutability =================================== //

// Variables are immutable by default
let a = 123;
// a = 456; // error: cannot assign twice to immutable variable `a`

// Variables can be made mutable
let mut b = 123;
b = 456; // ok

// Variables can be shadowed
let c = 123;
let c = c + 1;

// 2) ====================================== Data types =========================================== //
 
// Scalar types
let a: i32 = 123; // signed 32-bit integer
let b: u32 = 123; // unsigned 32-bit integer
let c: f32 = 123.0; // 32-bit floating point number

// Boolean
let d: bool = true;

// Character
let e: char = 'a';

// Compound types
let f: (i32, f64, u8) = (500, 6.4, 1); // Tuple
let g: [i32; 5] = [1, 2, 3, 4, 5]; // Array

// Difference between tuples and arrays in terms of storage on the stack is that when
// a bounded array is declared, the compiler knows exactly how much space it will take up. Tuples do
// not have that information because they could be anyt size up to the maximum.

// 3) ==================================== Functions =============================================== //

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 4) ==================================== Comments ================================================ //

// Single line comment

/*
Multi-line
comment
*/

/// Documentation comment

// 5) =================================== Control flow ============================================= //

let a = 3;

if a > 0 {
    println!("positive");
} else if a < 0 {
    println!("negative");
} else {
    println!("zero");
}

let b = if a > 0 {
    1
} else {
    -1
};

// 6) ========================================= Loops =============================================== //

let mut i = 0;

while i < 10 {
    println!("{}", i);
    i += 1;
}

for i in 0..10 {
    println!("{}", i);
}

// 7) ==================================== Ownership ================================================ //

let a = String::from("hello");
let b = a; // a has been moved to b

// println!("{}", a); // error: value borrowed here after move

let mut c = String::from("hello");
let d = &c; // borrow c

// & - borrow operator, i.e. they allow you to use values without taking ownership of them (immutable)
// for example &c could be c. Here it gives immutable access to the String while we do not take ownership (move) it.

// * - dereference operator (indirection) that takes a reference to a pointer and gives access to the value
// for example &mut c could be c then call *c = theValue

{
    let e = &mut c; // mutable reference
    *e = "changed".to_string();
}

println!("{}", c); // ok

let mut e = String::from("hello");
let f = &mut e; // mutable borrow of e

f.push_str(", world!"); // ok
println!("{}", e); // hello, world!

// 8) ================================ References and borrowing ===================================== //

let a = String::from("hello");
let b = a.clone(); // deep copy

println!("{}", a); // hello\

fn add_world(s: &String) {
    s.push_str(", world!");
    println!("{}", s); // hello, world!
}

let c = String::from("hello");
add_world(&c); // ok
// add_world(c); // ERROR: because function takes a ref doesn't mean ownership's taken

// 9) ===================================== Structs ================================================ //

struct Point {
    x: f64,
    y: f64,
}

let a = Point { x: 0.0, y: 0.0 };

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

let b = Point3D { x: 0.0, y: 0.0, z: 0.0 };


// impl - keyword to declare the implementation block for any given type right after defining the type
// syntax: impl <Name of the Type> {}
// it allows us to define methods(bind functions to a given data structure) in one place
// Note: params are the same as for functions
impl Point3D { 
    fn new(x: f64, y: f64, z: f64) -> Point3D {
        if (x < 0.0 || y < 0.0 || z < 0.0) {
            panic!("invalid coordinate!");
        }
        Point3D { x: x, y: y, z: z }
    }
}

let c = Point3D::new(0.0, 0.0, 0.0);

// 10) ===================================== Enums & match ======================================== //

enum Message {
    Quit, // have no data
    Move { x: i32, y: i32 }, // have anonymous struct
    Write(String), // have String
    ChangeColor(i32, i32, i32), // tuple of 3 i32s
}

let a = Message::Write(String::from("hello"));

match a {
    Message::Quit => {
        println!("The Quit variant has no data to destructure.")
    },
    Message::Move { x, y } => {
        println!("Move in the x direction {} and in the y direction {}", x, y);
    },
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
        println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    },
}

// 11) ===================================== Modules ================================================ //

// mod - keyword to declare modules which contain associated functions definitions
// syntax: mod <module_name> {}
// Note: modules are private by default, but this can be overridden using the pub keyword
mod sound { // start a module
    mod instrument { // start nested module (privacy can also be used here)
        fn echo(x: i32) -> i32 {
            x
        }
    }
}

// 12) ===================================== Object Oriented Programming ============================= //

// Structs are used to create custom data types.
// The primary difference between a tuple struct and a regular struct is that a tuple struct has no
// names associated with its fields, whereas a regular struct has named fields.

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Rectangle { p1: Point { x: x1, y: y1 },
                        p2: Point { x: x2, y: y2 } } = *self;

        // alternate syntax:
        // let Rectangle { p1, p2 } = *self;
        // let Point { x: x1, y: y1 } = p1;
        // let Point { x: x2, y: y2 } = p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        let Rectangle { p1: Point { x: x1, y: y1 },
                        p2: Point { x: x2, y: y2 } } = *self;

        let Rectangle { p1: Point { x: x3, y: y3 },
                        p2: Point { x: x4, y: y4 } } = *r;

        (x3 > x1 && x3 < x2 && y3 > y 1 && y3 < y2) &&
        (x4 > x1 && x4 < x2 && y4 > y 1 && y4 < y2)
    }

    fn square(p: Point, size: f64) -> Rectangle {
        Rectangle {
            p1: p,
            p2: Point { x: p.x + size, y: p.y - size },
        }
    }
}

fn main() {
    let black = Color { red: 0, green: 0, blue: 0 };
    let origin = Point { x: 0.0, y: 0.0 };

    let rect1 = Rectangle {
        p1: Point { x: origin.x, y: origin.y },
        p2: Point { x: 10.0, y: -10.0 },
    };

    let rect2 = Rectangle {
        p1: Point { x: origin.x, y: origin.y },
        p2: Point { x: 5.0, y: -5.0 },
    };

    let rect3 = Rectangle {
        p1: Point { x: 2.0, y: -2.0 },
        p2: Point { x: 7.0, y: -7.0 },
    };

    let sq = Rectangle::square(Point { x: 3.0, y: -3.0 }, 5.0);

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // pretty print

    println!("rect1 area is {}", rect1.area());
    println!("rect2 area is {}", rect2.area());
    println!("rect3 area is {}", rect3.area());
    println!("sq area is {}", sq.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sq? {}", rect1.can_hold(&sq));
}

// 13) ===================================== Error Handling ========================================= //

// panic! - macro that causes the current thread to panic
// syntax: panic!(<message>)
// Note: panic! unwinds the stack, cleans up the heap, and then exits
// Note: panic! is for unrecoverable errors

// std::panic::catch_unwind - function that catches a panic
// syntax: catch_unwind(|| <code>)
// Note: catch_unwind returns a Result<T, E> where T is the return type of the closure and E is the
//       panic payload
// Note: catch_unwind is for recoverable errors

// 14) ===================================== Generic Types ========================================= //

// Generics are a way of writing code that can be used for many different
// types without needing to be rewritten for each type.

// Generic Data Types
// syntax: <T> - type parameter
// Note: generic types are often named T, U, V, etc.

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// Generic Methods
// syntax: <T> - type parameter
// Note: generic methods are often named T, U, V, etc.

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generic Traits
// syntax: <T> - type parameter
// Note: generic traits are often named T, U, V, etc.

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,

    // ...
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.location)
    }
}

// x) ===================================== Better Example ========================================= //

// Here is a more complex example of a program that uses many of the concepts we’ve discussed so far.
// It’s a program that simulates a blog post. The blog post has a title, an author, and a body. The
// blog post also has a method to create a summary of the post. The summary is just the first line of
// the body of the post.

// Here’s the code:

struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

// 15) ===================================== Advanced Features ========================================= //

// 16) ===================================== Macros ========================================= //

// 17) ===================================== Crates and Modules ========================================= //

// 18) ===================================== Cargo and Crates.io ========================================= //

// 19) ===================================== Smart Pointers ========================================= //

// 20) ===================================== Concurrency ========================================= //

// 21) ===================================== The Rustonomicon ========================================= //

// 22) ===================================== Unsafe Rust ========================================= //

// 23) ===================================== Advanced Traits ========================================= //

// 24) ===================================== Advanced Types ========================================= //

// 25) ===================================== Final Project ========================================= //

// 26) ===================================== Appendix ========================================= //

// 27) ===================================== Index ========================================= //