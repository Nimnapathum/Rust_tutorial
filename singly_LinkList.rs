struct LinkList{
    head: pointer,
}
#[derive(Debug)]
struct Node{
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;

impl LinkList{
    fn new() -> LinkList{
        LinkList{head: None}
    }

    fn add(&mut self , element: i32){
        // match self.head{
        //     None =>{
        //         let new_node = Some(Box::new(Node{
        //             element: element,
        //             next: None,
        //         }));
        //         self.head = new_node;
        //     }
        //     Some(previous_head) =>{
        //         let new_node = Some(Box::new(Node{
        //             element: element,
        //             next: Some(previous_head),
        //         }));
        //         self.head = new_node;
        //     }
        // }

        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node{
            element: element,
            next: previous_head,
        }));
        self.head = new_head;

    }

    fn remove(&mut self) -> Option<i32>{
        match self.head.take(){
            Some(previous_head) =>{
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self){
        let mut list_traversal = &self.head;
        while !list_traversal.is_none(){
            println!("{:?}" , list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }

}
fn main(){
    // let list = LinkList{
    //     head: Some(Box::new(Node { 
    //         element: 1, 
    //         next: Some(Box::new(Node{
    //             element: 2,
    //             next: Some(Box::new(Node { 
    //                 element: 3, 
    //                 next: None 
    //             }))
    //         })) 
    //     }))
    // };

    // println!("{:#?}" , &list.head);

    let mut list = LinkList::new();
    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    //println!("{:#?}" , list.head);
    list.print();

    let removed = list.remove();
    //println!("{:#?}" , list.head)
    list.print();
}
