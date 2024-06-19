use std::rc::Rc;

enum List{
    Cons(i32 , Option<Rc<List>>),
}

fn main(){
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None)))));
    println!("{}" , Rc::strong_count(&a));
    {
        let b = List::Cons(3 , Some(Rc::clone(&a)));
        println!("{}" , Rc::strong_count(&a));
        let c = List::Cons(4 , Some(Rc::clone(&a)));
        println!("{}" , Rc::strong_count(&a));
    }
    println!("{}" , Rc::strong_count(&a));
}
