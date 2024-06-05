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

trait Draw{
    fn draw_object(&self){
        println!("Drawing the object");
    }
}

//super traits 
trait Shape : Draw{
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

impl Draw for Rectangle{
    fn draw_object(&self) {
        println!("Drawing object Rectangle {} {} " , self.height , self.width);
    }
}

impl Draw for Square{
    fn draw_object(&self) {
        println!("Drawing object Square {} {} " , self.size , self.size);
    }
}

fn shape_propertice_dynamin(object : Box<dyn Shape>){
    object.area();
    object.perimeter();
}

fn return_shape(dimention: Vec<f64>) -> Box<dyn Shape>{
    if dimention.len() == 1 {
        let sq = Square{
            size: dimention[0],
            info:drawing_info{
                line_width: 5,
                color: String::from("Black"),
            },
        };
        Box::new(sq)
    }else{
        let rect = Rectangle{
            height: dimention[0],
            width: dimention[1],
            info:drawing_info{
                line_width: 5,
                color: String::from("black"),
            },
        };
        Box::new(rect)
    }
}

fn main(){
    let square_1 = Square{size: 4.5 , info:drawing_info{line_width: 2 , color: String::from("Black")}};
    //square_1.area();
    //square_1.perimeter();

    let rec_1 = Rectangle{height: 2.5 , width: 3.0 , info:drawing_info{line_width: 2 , color: String::from("Blue")}};
    rec_1.area();
    rec_1.perimeter();

    shape_propertice_dynamin(Box::new(square_1));
}
