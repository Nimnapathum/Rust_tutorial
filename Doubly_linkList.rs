use std::{cell::RefCell, rc::Rc };
struct LinkList<T>{
    head: pointer<T>,
    tail: pointer<T>,
}

struct Node<T>{
    element: T,
    next: pointer<T>,
    previous: pointer<T>,
}

type pointer<T> = Option<Rc<RefCell<Node<T>>>>;

impl <T: std::fmt::Debug + std::marker::Copy + std::fmt::Display>LinkList<T>{
    
    fn new() -> Self{
        LinkList{
            head: None,
            tail: None,
        }
    }

    fn add(&mut self , element: T){
        let new_node = Node::new(element);

        match self.head.take(){
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_node);
            }
            None =>{
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    fn add_to_back(&mut self , element: T){
        let new_tail = Node::new(element);

        match self.tail.take(){
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().previous = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove(&mut self) -> Option<T>{
        if self.head.is_none(){
            println!("List is empty");
            None
        }else{
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head.take().map(|old_head| match old_head.borrow_mut().next.take(){
                Some(new_head) => {
                    new_head.borrow_mut().previous = None;
                    self.head = Some(new_head);
                    self.head.clone()
                }
                None => {
                    self.tail = None;
                    println!("List is empty");
                    None
                }
            });
            Some(removed_val)
        }
    }

    fn remove_from_back(&mut self) -> Option<T>{
        if self.tail.is_none(){
            println!("List is empty");
            None
        }else{
            let removed_val = self.tail.as_ref().unwrap().borrow().element;
            self.tail.take().map(|old_tail| match old_tail.borrow_mut().previous.take(){
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                    self.tail.clone()
                }
                None => {
                    self.head.take();
                    println!("List is empty");
                    None
                }
            });
            Some(removed_val)
        }
    }

    fn print(&self){
        let mut traversal = self.head.clone();
        while !traversal.is_none(){
            print!("{} " , traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
        println!();
    }
}

impl <T>Node<T>{
    fn new(element: T) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node { element: element, next: None, previous: None }))
    }
}
fn main(){
    let mut list = LinkList::new();
    list.add("saman");
    list.add("kamal");
    list.add("sunil");
    list.add("nimal");
    list.print();
    
    list.remove();
    list.print();
    
    list.add_to_back("anil");
    list.print();
    
    list.remove_from_back();
    list.print();

}
