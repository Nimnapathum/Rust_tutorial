use test1::{product::{Product , Category} , customer::Customer , order::Order};
use array_tool::vec::*;

fn main() {
    //let product_1 = Product::new(123 , String::from("Kettle") , 299.99 , Category::Electronics);
    //let customer_1 = Customer::new(102 , String::from("Nimna Pathum") , String::from("nimna123@gmail.com"));
    //let order_1 = Order::new(202 , product_1 , customer_1 , 32);
    //println!("The total cost of order {} is : {}" , order_1.id , order_1.total_bill());

    let product_1 = Product::new(1 , String::from("TV") , 105.99 , Category::Electronics);
    let product_2 = Product::new(2 , String::from("Radio") , 50.99 , Category::Electronics);
    let product_3 = Product::new(3 , String::from("wetlook") , 10.00 , Category::Clothing);
    let product_4 = Product::new(4 , String::from("Madol duwa") , 20.50 , Category::Books);

    let set_1 = vec![&product_1 , &product_2];
    let set_2 = vec![&product_2 , &product_3];
    let intersection = set_1.intersect(set_2);
    println!("product 1 and product 2 intersection : {:#?}" , intersection);

}
