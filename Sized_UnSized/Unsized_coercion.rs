fn str_slice_fn(s: &str){}

fn array_slice_fn<T>(s: &[T]){}

trait Some_trait{
    fn method(&self);
}

impl<T> Some_trait for [T]{
    fn method(&self) {}
}

fn main(){
    let some_string = String::from("String");
    str_slice_fn(&some_string);

    let slice: &[i32] = &[1];
    let vec: Vec<i32> = vec![1];
    let array = [1 , 2 , 3];

    array_slice_fn(slice);
    array_slice_fn(&vec); // Deref coercion
    array_slice_fn(&array); // Unsized coercion

    slice.method();
    vec.method();
    array.method();
}