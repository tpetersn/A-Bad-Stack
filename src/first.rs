use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}



impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node { 
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),//takes whats at the head of the list (self.head) and store it in next
            //at the same time, were going to replace the head element of our list object with Link::Empty
        });
        self.head = Link::More(new_node);//updates the head to be our more variant of he link which is a Box
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace( &mut self.head, Link::Empty) {
            Link::Empty =>  None, //list is empty it returns None
            Link::More(node) => { //if the list is not empty it returns some and the first value of the list 
                self.head = node.next; //replace head of the list with the second item in the list
                Some(node.elem)
            }   
        }
   
    }
}


