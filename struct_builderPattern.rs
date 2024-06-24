#[derive(Debug , Default , Clone)]
struct Customer{
    name: String,
    username: String,
    membership: Membershiptype,
    gender: char,
    country: String,
    age: u8,
}

#[derive(Debug , Clone)]
enum Membershiptype{
    new,
    causual,
    loyal,
}

impl Default for Membershiptype{
    fn default() -> Self{
        Membershiptype::new
    }
}

impl Customer{
    fn new(name: String) -> Customerbuilder{
        Customerbuilder{
            name: name,
            ..Default::default()
        }
    }
//     fn new(name: String) -> Self{
//         Customer{
//             name: name,
//             ..Default::default()
//         }
//     }

//     fn new_2(name: String , username: String) -> Self{
//         Customer{
//             name: name,
//             username: username,
//             ..Default::default()
//         }
//     }

//     fn new_3(name: String , username: String , membershiptype: Membershiptype) -> Self{
//         Customer { 
//             name: name, 
//             username: username, 
//             membership: membershiptype, 
//             ..Default::default() 
//         }
//     }
}

#[derive(Debug , Default)]
struct Customerbuilder{
    name: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<char>,
    country: Option<String>,
    age: Option<u8>,
}

impl Customerbuilder{
    fn username(&mut self , username: String) -> &mut Self{
        self.username = Some(username);
        self
    }

    fn membership(&mut self , membership: Membershiptype) -> &mut Self{
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self , gender: char) -> &mut Self{
        self.gender = Some(gender);
        self
    }

    fn country(&mut self , country: String) -> &mut Self{
        self.country = Some(country);
        self
    }

    fn age(&mut self , age: u8) -> &mut Self{
        self.age = Some(age);
        self
    }

    fn build(&mut self) -> Customer{
        Customer{
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.gender.unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
        }
    }
}

fn main(){
    // let user1 = Customer::new("Nimna".to_string());
    // let user2 = Customer::new_2("Pathum".to_string(), "MyUserName".to_string());
    // let user3 = Customer::new_3("Eshan".to_string(), "UserName".to_string(), Membershiptype::causual);

    let user_1 = Customer::new("Nimna".to_string()).build();
    let user_2 = Customer::new("pathum".to_string()).username("UserName".to_string()).build();
    let user_3 = Customer::new("Eshan".to_string()).username("UserName".to_string()).membership(Membershiptype::loyal).build();
    println!("{:#?}" , user_1);
    println!("{:#?}" , user_2);
    println!("{:#?}" , user_3);
}
