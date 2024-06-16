struct Book{
    title: String,
    author: String,
    genre: String,
}

struct BookIterator{
    properties: Vec<String>,
}

impl Iterator for BookIterator{
    type Item = String;
    fn next(&mut self)-> Option<Self::Item>{
        if !self.properties.is_empty(){
            Some(self.properties.remove(0))
        }else{
            None 
        }
    }
}

impl IntoIterator for Book{
    type Item = String;
    type IntoIter = BookIterator;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator{
            properties: vec![self.title , self.author , self.genre],
        }
    }
}
fn main(){
    let book = Book{
        title: "Madol duwa".to_string(),
        author: "Martin Wikramasinghe".to_string(),
        genre: "fantasy".to_string(),
    };

    let mut book_iterator = book.into_iter();
    // println!("{:?}" , book_iterator.next());    
    // println!("{:?}" , book_iterator.next());    
    // println!("{:?}" , book_iterator.next());    
    // println!("{:?}" , book_iterator.next());    
    // println!("{:?}" , book_iterator.next());    

    for book_details in book_iterator{
        println!("{book_details}");    

    }
}
