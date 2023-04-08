
#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8
}

fn main() {
    let name = "p";
    let age = 12;
    let p = Person{name, age};
    println!("Hello {:?}", p);    
}

