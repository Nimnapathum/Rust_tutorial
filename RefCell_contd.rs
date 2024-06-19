use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct File{
    user_count: u32,
}

#[derive(Debug)]
struct User{
    file: Rc<RefCell<File>>,
}

fn main(){
    let tex_file = Rc::new(RefCell::new((File{user_count: 0})));


    let user1 = User{
        file: Rc::clone(&tex_file),
    };
    user1.file.borrow_mut().user_count += 1;
    println!("{:?}" , tex_file.borrow().user_count);


    let user2 = User{
        file: Rc::clone(&tex_file),
    };
    user2.file.borrow_mut().user_count += 1;
    println!("{:?}" , tex_file.borrow().user_count);
}
