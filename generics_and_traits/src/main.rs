// fn main() {
//     //methods of traits
//     //1.Concrete Method
//     //2.Abstract Method
//     //Declare a Trait
//     //#[allow(dead_code)]
//     /*trait TraitName{
//         fn method(&self) {
//             //body
//         }
//     }
//     struct Point {
//         x: i32,
//         y: i32,
//     }
//     //implementation in a trait
//     imp TraitName for Point {
//         fn method(&self) {
//         //body
//         }
//     } */
//     //create an instance of the structure
//     let c = Circle { radius: 2.0 };

//     let r = Rectangle {
//         width: 2.0,
//         height: 2.0,
//     };
//     println!("Area of Circle{}", c.shape_area);
//     println!("Area of Rectangle{}", r.shape_area);
// }

// struct Circle {
//     radius: f32,
// }
// struct Rectangle {
//     width: f32,
//     height: f32,
// }
// //declare a trait
// trait Area {
//     fn shape_area(&self) -> f32;
// }
// //implement the trait
// impl Area for Circle {
//     fn shape_area(&self) -> f32 {
//         3.14 * self.radius * self.radius
//     }
// }
// impl Area for Rectangle {
//     fn shape_area(&self) -> f32 {
//         self.height * self.width
//     }
// }

use std::fmt::{format, Display};

//Generics
//Ex-1
fn main() {
    println!("- Passing a string literal");
    concatenate("Rust", "Programmin");
    println!("Passing a initeger");
    concatenate(10 as i32, 1 as i32);
}
fn concatenate<T: Display>(t: T, s: T) {
    let result = format!("{}{}", t, s);
    println!("{}", result)
}
fn somethin() {
    let square = |x: i32| -> i32 {};
}
