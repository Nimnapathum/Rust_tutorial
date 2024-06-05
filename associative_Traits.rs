#[derive(Debug)]
struct kmh{value: f64,}
#[derive(Debug)]
struct km{value: f64}
#[derive(Debug)]
struct miles{value: f64}
#[derive(Debug)]
struct mlh{value: f64}

trait Distance{
    type distance;
    fn calc_distance(&self) -> Self::distance;
}

impl Distance for kmh{
    type distance = km;
    fn calc_distance(&self) -> Self::distance {
        Self::distance{ 
            value: self.value * 3.0
        }
    }
}

impl Distance for mlh{
    type distance = miles;
    fn calc_distance(&self) -> Self::distance {
        Self::distance{
            value: self.value * 3.0
        }
    }
}
//Associative traits
fn main(){
    let miles_1 = mlh{value: 5.0};
    println!("Distance after 3 hours: {:?}" , miles_1.calc_distance());
}
