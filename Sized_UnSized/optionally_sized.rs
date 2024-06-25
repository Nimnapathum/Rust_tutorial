use std::fmt::Debug;
struct UnSizedStruct<T: ?Sized>{
    sized_field: i32,
    unsized_field: T,//unsized field must be last field
}

fn print_fn<T: Debug + ?Sized>(t: &T){
    println!("{:?}" , t);
}
fn main(){
    let x = UnSizedStruct{
        sized_field: 3,
        unsized_field: [3],
    };

    let x = "My name";
    print_fn(&x);
}