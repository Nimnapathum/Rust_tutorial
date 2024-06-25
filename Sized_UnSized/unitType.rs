struct User;
struct Admin;

trait Authenticate{
    fn authenticate(&self , username: &str , password: &str)->bool;
}

impl Authenticate for Admin{
    fn authenticate(&self , username: &str , password: &str)->bool {
        username == "admin" && password  == "adminpass"
    }
}

impl Authenticate for User{
    fn authenticate(&self , username: &str , password: &str)->bool {
        username == "user" && password == "userpass"
    }
}

fn login<T: Authenticate>(role: T, username: &str , password: &str)->bool{
    role.authenticate(username, password)
}
fn main(){
    let admin = Admin;
    let user = User;
    let admin_login = login(admin, "admin", "adminpass");
    let user_login = login(user, "user", "userpass");

    if admin_login{
        println!("Admin login successfully!");
    }else{
        println!("User login successfully!");
    }

    let x = ();
    let y = ();
    let z = ();

    struct ABC;
    let x = ABC;
    let y = &x;
    let z = x;
} 
