// generics
struct Point<T , U>{
    x: T,
    y: U,
}

impl<T , U> Point<T , U>{
    fn new_point(x: T , y: U) -> Point<T , U>{
        Point{x , y}
    }
}

impl Point<u32 , u32>{
    fn add_points(p1: &Point<u32 , u32> , p2: &Point<u32 , u32>) -> Point<u32 , u32>{
        Point{x:(p1.x + p2.x) , y:(p1.y + p2.y)}
    }

    fn printing(&self) {
        println!("Points: {} , {}" , self.x , self.y)
    }
}

impl Point<f64 , f64>{
    fn printing(&self) {
        println!("Points: {} , {}" , self.x , self.y)
    }    
}

fn main() {
    let point_1 = Point::new_point(3, 4);
    let point_2 = Point:: new_point(6, 7);
    let point_3 = Point::add_points(&point_1, &point_2);
    point_1.printing();
    point_2.printing();
    point_3.printing();

    let point_4 = Point::new_point(3.5, 5.5);
    point_4.printing();
}
