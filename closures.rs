struct User{
    name: String,
    age: u32,
}

// fn validate_user(name : &str) -> bool{
//     name.len() != 0
// }

fn is_valid_user<V1>(name: &str , validate_user_in: V1) -> bool
where
    V1: Fn(&str) -> bool,
{
    validate_user_in(name)
}

fn main(){
    let user_1 = User{
        name: String::from("Someone"),
        age: 22,
    };

    let validate_user = |name: &str| name.len() != 0;
    //println!("User_1 is {}" , validate_user(&user_1.name));

    println!("User validity {}" , 
        is_valid_user(&user_1.name, validate_user)
    );
}
