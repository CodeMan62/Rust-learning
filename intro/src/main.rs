use core::panic;

fn main() {
    //Variables
    //variable making and giving it a type and adding mutability
    let mut x: i32 = 10;
    //immutability
    x += 1;
    //pattern to declare a variables
    let (a, b) = ("foo", 12);

    //Expresions
    //Everything that returns a value
    //fn foo() ->i32{ 5 }
    let y = if x > 10 { "greater" } else { "False" };
    println!(" x = {} is {} less than zero", x, y);
    //"{}" is Rust's(most basic) string interpolation operator
    //Array
    //Two types to write
    let arr = [1, 2, 3]; //(array of three elements)
    let arr = [1; 20]; //(array of 20 '1's)

    //Slice
    let arr = [1, 2, 3, 4, 5];
    let total_slice = &arr; //slice all arr
    let total_slice = &arr[..]; //Same , but more explicit
    let partial_slice = &arr[1..3]; //[2,3,4]

    //Strings
    //Two types
    //1.string
    //2.&str
    let s: &str = "Hello";
    let s2: String = "Hello".to_string();
    let s3: String = String::from("Hello");
    let s4: &str = &s3;

    //Tuples
    let foo = (72, 'H', 5.1);
    let (x, y, z) = (72, 'H', 5.1);
    let (a, b, c) = foo; // a = 72, b = 'H', c = 5.1

    //Casting
    let a: i32 = 10;
    let b: u32 = a as u32;

    //VECTOR
    //Vec<T>
    //A standard library type: you don't need to import anything.
    //A Vec (read "vector") is a heap-allocated growable array.
    //(cf. Java's ArrayList, C++'s std::vector, etc.)
    //<T> denotes a generic type.
    //The type of a Vec of i32s is Vec<i32>.
    //Create Vecs with Vec::new() or the vec! macro.
    //Vec::new() is an example of namespacing. new is a function defined for the Vec struct.
    //Code
    //explicit typing
    let mut v0 = Vec::new();
    //2 diffrent types
    v0.push(1);
    v0.push(2);
    v0.push(3);

    let v1 = vec![1, 2, 3];
    //v0 and v1 both are Same
    //two more types
    let v2 = vec![0; 4];
    let v3 = vec![0, 0, 0, 0];
    //v2 and v3 both are Same
    //Accesing the vector
    let v1v = v1[1]; //2
                     //Refrences
                     //Reference types are written with an &: &i32.
                     //References can be taken with & (like C/C++).
                     //References can be dereferenced with * (like C/C++).
                     //References are guaranteed to be valid.
                     //Validity is enforced through compile-time checks!
                     //These are not the same as pointers!
                     //Reference lifetimes are pretty complex, as we'll explore later on in the course.
    let mut r = 12;
    let ref_r = &r;
    println!("{}", *ref_r); //12
                            //Control Flow
                            //IF Statements
                            // if  r > 0 {
                            //     12
                            // } else if x == 0 {
                            //     0
                            // } else {
                            //     println!("nothing");
                            // }
                            //Loops
                            //While loop
    while r > 5 {
        r += 1;
        println!("{}", r);
        if r <= 100 {
            break;
        }
    }
    //another type
    loop {
        r += 1;
        println!("{}", r);
        if r <= 100 {
            break;
        }
    }
    //For loop
    for r in 0..10 {
        println!("{}", r);
    }
    let xs = [0, 1, 2, 3, 4];
    // Loop through elements in a slice of `xs`.
    for x in &xs {
        println!("{}", x);
    }

    //Print! & Println!
    //>>Print stuff out YaY
    //Use {} for general string interpolation, and {:?} for debug printing.
    //Some types can only be printed with {:?}, like arrays and Vecs.
    print!("{}, {}, {}", "foo", "3", true);
    println!("{:?}, {:?}", "foo", [1, 2, 3]);

    //Format!
    //Uses println!-style string interpolation to create formatted Strings.
    let fmmtd = format!("{}, {:x}, {:?}", 12, 155, Some("Hello"));
    println!("{}", fmmtd);

    //panic!(msg)
    //Exits current task with given message.
    //Don't do this lightly! It is better to handle and report errors explicitly.
    if r < 0 {
        panic!("Ohh no")
    }
    //assert! & assert_eq!
    // assert!(condition) panics if condition is false.
    // assert_eq!(left, right) panics if left != right.
    // Useful for testing and catching illegal conditions.
    #[test]
    fn test_something() {
        let actual = 1 + 2;
        assert!(actual == 3);
        assert_eq!(3, actual);
    }
    // unreachable!()
    // Used to indicate that some code should not be reached.
    // panic!s when reached.
    // Can be useful to track down unexpected bugs (e.g. optimization bugs).
    if false {
        unreachable!();
    }
    //     unimplemented!()
    // Shorthand for panic!("not yet implemented")
    // You'll probably see this in your homework a lot!
    fn sum(x: Vec<i32>) -> i32 {
        // TODO
        unimplemented!();
    }
    //Match Statament
    let m = 3;

    match m {
        1 => println!("one fish"), // <- comma required
        2 => {
            println!("two fish");
            println!("two fish");
        } // <- comma optional when using braces
        _ => println!("no fish for you"), // "otherwise" case}
    }
    //_ is Haskell or ocaml
    let n = 3;
    let p = -3;
    match (n, p) {
        (1, 1) => println!("one"),
        (2, j) => println!("two, {}", j),
        (3, 3) => println!("three"),
        (i, j) if i > 5 && j < 0 => println!("On guard!"),
        (_, _) => println!(":<"),
    }
}
