fn main(){
    let some_product = Some("laptop");
    let mut products = vec!["phones" , "battery" , "charger"];

    // match some_product{
    //     Some(product) => products.push(product),
    //     _ => {}
    // };

    // if let Some(product) = some_product{
    //     products.push(product);
    // }

    //products.extend(some_product);

    // let prod_iter = products.iter().chain(some_product.iter());

    // for prod in prod_iter{
    //     println!("{:?}" , prod);
    // }

    let prods = vec![Some("charger") , Some("battery") , None , Some("phone")];
    // let mut prod_without_none = Vec::new();
    // for p in prods{
    //     if p.is_some(){
    //         prod_without_none.push(p.unwrap());
    //     }
    // }

    //let prod_without_none = prods.into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<&str>>();
    let prod_without_none = prods.into_iter().flatten().collect::<Vec<&str>>();
}