#[derive(Debug , Default)]
pub struct Student{
    pub id: u8,
    pub age: u8,
    pub name: String,
}

impl Student{
    pub fn new(name: String) -> Result<Self , String>{
        if name.chars().all(|x| matches!(x , 'a'..='z')){
            Ok(Self{
                id: 0,
                age: 0,
                name: name ,
            })
        }else{
            Err("The name is invalid".to_string())
        }       
    }
}

// impl Default for Student{
//     fn default() -> Self{
//         Self { id: 0, age: 20, name: "Unknown".to_string() }
//     }
// }