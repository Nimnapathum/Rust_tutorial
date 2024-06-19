use std::{cell::RefCell , rc::Rc};
fn main(){
    //part 1
    // let mut x = 50;
    // let x1 = &x;
    // let x2 = &x;
    // let x3 = &mut x;
    // println!("{} {}" , x1 , x2);

    //part 2
    let a = RefCell::new(10);
    {
        let b = a.borrow();
        let c = a.borrow();
    }
    // drop(b);
    // drop(c);
    let d = a.borrow_mut();
    drop(d);
    //println!("{} {}" , b , c);
    println!("{:?}" , a);

    //part 3
    let x = RefCell::new(10);
    let mut y = x.borrow_mut();
    *y = 15;
    drop(y);
    println!("{:?}" , x);

    //part 4
    let s = Rc::new(RefCell::new(String::from("Hi")));
    let t = Rc::clone(&s);

    *t.borrow_mut() = String::from("There");
    println!("{:?}" , s);
}
