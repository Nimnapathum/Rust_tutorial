/*use std::vec;

fn main(){
    let vec1: Vec<i32> = vec![2 , 3 , 5];
    let vec = ownership(vec1);
    println!("Vec is : {:?}", vec);
    //println!("Vec1 is : {:?}" , vec1)

}

fn ownership(mut vec: Vec<i32>) -> Vec<i32>{
    vec.push(10);
    vec
}
*/
/*
fn main(){
    let mut number = 64;
    let mut ref_1 = &mut number;
    let deref_copy = *ref_1;
    *ref_1 = 32;
    println!("{number}  {deref_copy}");

    let mut vec_1 = vec![2 , 3 , 4];
    let ref_1 = &mut vec_1;
    let vec_2 = ref_1.clone();
}
*/

/*
struct Car{
    owner: String,
    model_number: i32,
    litre: Option<i32>
}

impl Car{
    fn my_info(&self){
        println!("Owner Name: {} Model No: {} Litre: {}" , self.owner , self.model_number , self.litre);
    } 

    fn add_fuel(&mut self , lit: i32){
        self.litre += lit;
    }

    //associative funtions.does not get self.call like Car::name
    fn insuarence_price() -> i32{
        10000
    }

    fn selling_price(self) -> i32{
        self.litre + Car::insuarence_price()
    }

    fn new(name: String , no: i32) ->Self{
        Self{
            owner : name,
            model_number : no,
            litre : 0
        }
    }

}


fn main(){
    let mut mycar = Car{
        owner: String::from("Nimna"),
        model_number: 23422,
        litre: 3
    };
    //println!("My car {}" , mycar.owner);  
    //let name = mycar.owner;  
    mycar.my_info();
    mycar.add_fuel(10);
    mycar.my_info();

    let newcar = Car::new(("Pathum").to_string(), 3999);
    newcar.my_info();
}
*/
/*
enum vehicle{
    Car,
    Van,
    Bus
}

impl vehicle{
    fn print_name(&self) -> String{
        let type_of : String = match self{
            vehicle::Car => "Car".to_string(),
            vehicle::Bus => "Bus".to_string(),
            vehicle::Van => "Van".to_string()
        };
        type_of
    }
}
fn main(){
    let mycar = vehicle::Bus;
    println!("{}" , mycar.print_name());

}
*/
use std::collections::HashMap;
fn main(){
    let list_of_int = vec!(3,6,4,3,6,2,5,6,4,7);
    let mut name: HashMap<i32 , i32> = HashMap::new();

    for i in &list_of_int{
        let freq = name.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}" , name);
}
