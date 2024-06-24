use std::default;

use structs::Student;
fn main() {
    let std1 = Student::new("nimna".to_string()).unwrap_or_default();
    println!("{:?}" , std1);

    let std2 = Student::default();
    println!("{:?}" , std2);

    let std3 = Student{
        age: 21,
        ..Default::default()
    };
    println!("{:?}" , std3);
}
