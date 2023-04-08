use crate::list::*;

// make an enum of types of nodes in a list
enum list {
    Node(i32, Box<list>),
    EndNode,
}

// make functions for linked list
impl list{
    fn new() -> list {
       EndNode 
    }

    fn link(self, elem: i32) -> list{
        //return the address of the new node after making it 
         Node(elem, Box::new(self))  
    }

    fn len(&self) -> u32 {
        //make a recursive to find the length of the linked list
        match *self {
            // a new node with optional arg for the value it holds, tail is a reference to trailing node of the list
            Node(_, ref next) => 1 + next.len(),
            //ending condition
            EndNode => 0,
        }
    }

    fn to_string(&self) -> String {
        //format! -> return a string 
        match *self {
            Node(val, ref next) => format!("{0}, {1}", val, next.to_string()),
            EndNode => format!("Null"),
        }
    }
}

fn main() {
    //make a new list
    //access impl functions using ::
    //mut allows for variables to be able to be changed
    let mut l = list::new(); 
    let mut _l1 = list::new();

    l = l.link(2);
    l = l.link(4);
    l = l.link(6);

    println!("len: {}", l.len());
    println!("{}", l.to_string());

}
