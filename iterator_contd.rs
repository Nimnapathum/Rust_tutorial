use std::collections::HashMap;
fn main(){
    let mut vec1 = vec![45 , 50 , 35 , 75 , 57 , 32];

    for values in &vec1{
        println!("{values}");
    }

    let mut person: HashMap<String , i32> = HashMap::new();
    person.insert("Nimna".to_string(), 22);
    person.insert("vinuka".to_string() , 14);
    person.insert("rashmi".to_string() , 18);

    for (name , age) in person{
        println!("{name} {age}");
    }

    //combinators

    let words = vec!["apple" , "orange" , "grape" , "pineapple" , "mango"];
    let mut result = vec![];

    for word in &words{
        if word.starts_with("a") || word.starts_with("o"){
            let uppercase = word.to_uppercase();
            result.push(uppercase)
        }
    }
    println!("Result: {:?}" , result);

    let result = words.into_iter().filter(|&word|word.starts_with("a") || word.starts_with("o")).map(|word| word.to_uppercase()).collect::<Vec<String>>();
    println!("{:?}" , result);
}