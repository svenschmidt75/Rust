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
        let new_node = Node {
            elem: elem,

            // We cannot just say 'next: self.head', because Rust would
            // move self.head to next, and then self.head would be
            // dangling. This is the very thing Rust wants to avoid.
            // Note that the fact that self is '&mut self' is irrelevant
            // here and has nothing to do with it. self is mutably
            // *borrowed*, that means push has to give self back.
            // Rust guarantees that self is consistent, even though
            // it is '&mut self'! 
            // The error message would be 'cannot move out of borrowed context',
            // which refers to 'self.next' (move-semantics) an the context is
            // self.
//            next: self.head,
            
            // swap
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match 'self.head' would capture by move, i.e. it
        // would invalidate self.head, which is not allowed
        // by Rust.
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.elem)
            }
        } 
    }
}

fn main() {
    println!("Hello, world!");
}
