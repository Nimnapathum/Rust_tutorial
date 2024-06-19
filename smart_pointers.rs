// #[derive(Debug)]
// enum List{
//     Cons(i32 , Box<List>),
//     Nil,
// }

#[derive(Debug)]
enum List{
    Cons(i32 , Option<Box<List>>)
}

fn main(){
    // let list = List::Cons(
    //     1,
    //     Box::new(List::Cons(2, Box::new(List::Cons(3 , Box::new(List::Nil)))))
    // );
    // println!("{:?}" , list);

    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3 , None))))))
    );
    println!("{:?}" , list);
}
