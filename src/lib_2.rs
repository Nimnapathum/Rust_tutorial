mod Shapes{
    pub struct Square{
        size: i32
    }

    impl Square{
        pub fn new(size: i32) -> Square{
            Square{size}
        }

        pub fn check_this_can_holding_other(&self , square: &Square) -> bool{
            self.size > square.size
        }

        pub fn new_2(size: i32) -> Square{
            match size{
                ..=0 => panic!("size should be positive"),
                _ => Square{ size }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use Shapes::Square;

    use super::*;
    /*
    #[test]
    fn can_holding(){
        let small_square = Square::new(3);
        let large_square = Square::new(5);
        assert_eq!(small_square.check_this_can_holding_other(&large_square) , false);
    }

    #[test]
    fn cannot_holding(){
        let small_square = Square::new(3);
        let large_square = Square::new(5);
        assert_eq!(small_square.check_this_can_holding_other(&large_square) , false);

    }
    */

    #[test]
    #[should_panic]
    fn is_panicking(){
        //let squre = Square::new(3);
        let s = Square::new_2(-3);
    }

    #[test]
    #[ignore]
    fn this_test_should_ignore(){
        println!("No need of this");
    }
}