struct drawing_info{
    line_width: u32,
    color: String,
}

struct Square{
    size: f64,
    info: drawing_info,
}

struct Rectangle{
    height: f64,
    width: f64,
    info: drawing_info,
}

trait Shape{
    fn area(&self) -> f64;
    fn perimeter(&self){
        println!("This is default messege");
    }
}

impl Shape for Square{
    fn area(&self) -> f64{
        let area = self.size * self.size;
        println!("Area of size {} Square is : {}" , self.size , area);
        area
    }

    fn perimeter(&self) {
        println!("this is specific for square");
    }
}

impl Shape for Rectangle{
    fn area(&self) -> f64{
        let area = self.height * self.width;
        println!("Area of size {} , {} Reqtangle is : {}" , self.height , self.width , area);
        area
    }
}

fn main(){
    let square_1 = Square{size: 4.5 , info:drawing_info{line_width: 2 , color: String::from("Black")}};
    square_1.area();
    square_1.perimeter();

    let rec_1 = Rectangle{height: 2.5 , width: 3.0 , info:drawing_info{line_width: 2 , color: String::from("Blue")}};
    rec_1.area();
    rec_1.perimeter();
}
