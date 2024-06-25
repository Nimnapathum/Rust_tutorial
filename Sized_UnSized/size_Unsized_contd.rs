use std::mem::size_of;

trait Shape {
    fn print(&self);
}

#[derive(Debug)]
struct Rectangle {}
#[derive(Debug)]
struct Circle {}
impl Shape for Rectangle {
    fn print(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Circle {
    fn print(&self) {
        println!("{:?}", self);
    }
}
fn main() {
    println!(
        "Size of a reference to sized type: {}",
        size_of::<&[i32; 3]>()
    );
    println!(
        "Size of a reference to Unsized type: {}",
        size_of::<&[i32]>()
    );

    let num_1 = &[10, 20, 30];
    let num_2: &[i32] = &[10, 20, 30];

    println!("Size of &Circle is: {}" , size_of::<&Circle>());
    println!("Size of &Rectangle is: {}" , size_of::<&Rectangle>());
    println!("Size of &dyn Shape is: {}" , size_of::<&dyn Shape>());

}
