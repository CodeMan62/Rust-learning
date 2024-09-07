// use std::fmt::Binary;

// //Struct
// fn main() {
//     println!("Hello, world!");
// mod foo {
// }
// struct Unit;
// let u = Unit;
//     pub struct Point {
//         pub x: i32,
//         y: i32,
//     }

//     //creates and returns a new Point
//     pub fn new(x: i32, y: i32) -> Point {
//         Point { x: x, y: y }
//     }
// }

// enum Resultinsh {
//     Ok,
//     Warning { code: i32, message: String},
//     Err(String)
// }
//let boxed_five = Box::new(26);

// enum list {
//     Nil,
//     Cons(i32, Box<list>);
// }

//Methods
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Point {
//     pub fn distance(&self, other: Point) -> f32 {
//         let (dx, dy) = (self.x - other.x, self.y - other.y);
//         ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
//     }
// }

// fn main() {
//     let p = Point { x: 1, y: 2 };
//     p.distance();
// }
struct Pizza(Vec<i32>);
struct PizzaSlice<'a> {
    pizza: &'a Pizza, // <- references in structs must
    index: u32,       //    ALWAYS have explicit lifetimes
}
struct PizzaConsumer<'a, 'b: 'a> {
    slice: PizzaSlice<'a>, //currently eating this one
    pizza: &'b Pizza, // <- so we can get more pizza
}

fn get_another_slice(c: &mut PizzaConsumer, index: u32) {
    c.slice = PizzaSlice { pizza: c.pizza, index: index};
}

let p = Pizza(vec![1,2,3]);
{
    let s = PizzaSlice { pizza: &p, index: index };
    let mut c = PizzaSlice { sli}
}
