fn main(){
    let fixed_string: &str = "This has fixed size";
    let mut flexible_string: String = String::from("This can grow");
    flexible_string.push(ch: 's');
}

/*fn main() {
    let x = 10;
    println!("Outter x in first is {x}");
    {
        let x = 40;
        println!("Inner x is {x}");
    }
    let x = 30;
    println!("Outter x in last is {x}")
}
*/
/*
fn main(){
    //&str and String
    let fixed_string: &str = "This has fixed size";
    let mut flexible_string: String = String::from("This can grow");
    flexible_string.push(ch: 's');

    // Arrays
    let mut array_1: [i32; 5] = [4, 5, 6, 7, 8];
    let num = array_1[3];
    let array_2: [i32; 10] = [0; 10]; // all indexes contain 0

    // Vectors
    let vec_1: Vec<i32> = vec![4, 5, 6, 7, 8];
    let num = vec_1[3];

    // Tuples
    let my_info: (&str, i32, &str, i32) = ("Salary" , 40000 , "Age" , 40);
    let salary_value = my_info.1;
    let (salary , salary_value, age , age_value) = my_info;
}
*/

fn main(){
    //my_function("Nimna");
    //let name = "Pathum";
    //my_function(name);

    //let result = basic_math(2, 3);
    //println!("{:?}" , result);

    //let (multi , sum, diff) = basic_math(4, 5);
    //println!("{multi} {sum} {diff}");

    let full_name: String = {
        let first_name = "Nimna";
        let last_name = "Pathum";
        format!("{} {}" , first_name , last_name)
    };
    println!("{full_name}");

}

/*
fn my_function(s: &str){
    println!("{s}");
}

fn basic_math(num1: i32 , num2: i32) -> (i32 , i32 , i32){
    (num1 * num2 , num1 + num2 , num1 - num2)
}
*/
