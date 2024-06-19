struct Huge_data;
struct Small_data;

trait Storage{}
impl Storage for Huge_data{}
impl Storage for Small_data{}

fn main(){
    let data_1 = Huge_data;
    let data_2 = Box::new(Huge_data);

    let data_3 = data_1;
    let data_4 = data_2;

    let data_5 = Box::new(Small_data);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3) , data_4 , data_5];
}
